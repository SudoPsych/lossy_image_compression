# lossy_image_compression

This Rust program performs compression of an image into a byte file and decompression of a
byte file into an image. The method of compression involves a discrete cosine transformation of
2x2 blocks of pixels, converting those values into integers and packing the bits of those integers
into 4 bytes that are compacted together and output to a file. The decompression process reverses these steps
and outputs an image that should look very similar to the original image before compression.

## Original Image
![fruit](https://github.com/SudoPsych/lossy_image_compression/blob/main/fruit.png?raw=true)
## Image after compression and decompression
![fruit_compressed](https://github.com/SudoPsych/lossy_image_compression/blob/main/fruit_compressed.png?raw=true)

## Usage
```bash
rpeg -c [filename]
rpeg -d [filename]
```
## Dependencies - Rust Crates

> csc411_image
>
> csc411_arith
> 
> csc411_rpegio
> 
> approx

# Details

## Structure
The program is compiled in the rpeg directory, the main file in this directory
contains the command line parsing and calls the compression and decompression
functions that live in codec.rs. The codec file contains the implementation of
compression and decompression along with the functions for getting and setting the
pixel values. In addition, it contains functions for packing the image data into bytes and
unpacking them from bytes. These packing functions invoke the functions created in the
bitpack crate that are in charge of directly manipulating the values stored within the
bytes. Functions in charge arithmetic are contained in the dct_trans file that lives within
the rpeg source directory. These functions are imported into and invoked in the codec
file for compression and decompression.

## Compression
The compression function receives an image file and reads in the data into an
RgbImage struct. A for loop begins and runs for enough iterations to loop over every
2x2 chunk of pixels in the image. Within the loop, the 4 pixels of the current 2x2 block
are located and the rgb values from each pixel are copied and collected into a vector
using. This rgb data is then converted into YPbPr form. The 4 y values are pushed into
a vector while the 4 pb and pr values are condensed into a single average value for
each of them. The y values are then fed through the discrete cosine transformation
function which performs the necessary arithmetic to get the coefficients of the 4 different
cosine waves. The values returned from this function are then quantized while the pb
and pr averages are converted to an index using the csc_411arith crate. These 6 values
are then packed into a 32 bit integer, converted to raw bytes in big endian order and
pushed into a vector. The loop repeats on the next block until the image has been
exhausted and the bytes are written to standard output.

## Decompression
The decompression function reads in a file containing the width and height of an
image along with the compressed data in the form of big endian ordered raw bytes. A
csc411_image is instantiated that will be modified with the decompressed values. A loop
begins that iterates through each 2x2 block of what will be the final image. The relevant
bytes are taken from the vector and the values are extracted from them using the
bitpack functions. The cosine coefficients are ‘dequantized’ (expanded) into roughly
their original values and the rough values of pb and pr are retrieved based on their
indices (that were stored in the word). The cosine coefficients are then fed through the
inverse discrete cosine transformation to roughly get the original brightness value of
each of the 4 pixels. The brightness values and the pb and pr averages are used to
calculate the rgb values of each pixel. The aforementioned image is then modified with
these values and the loop continues until all bytes have been exhausted. The image is
written to standard output.
## Bitpacking

> fiti and fitu:

These functions check whether the given value can fit within the specified width.
This is checked by shifting the bits to the left until the width specified is left aligned and
then shifted back so the width is right aligned. The resulting value of these shifts will be
equivalent to the input value if and only if the value fits inside of that width and will not
be equivalent if and only if it does not fit in the specified width.

> geti and getu:

These functions retrieve a series of bits (specified by width) from the middle of a
given word. This is done by shifting the relevant bits in the word to be left aligned. The
lsb must be taken into account to do this properly. The bits are then shifted to the right
until they are right aligned. Only the relevant bits in question will remain after these
shifts and the bits will be in the correct spot to represent the number it is meant to. For
unsigned integers and signed positive integers bits to the left of the value will be 0. For
signed negative integers, the left most bit would be a one when left aligned and would
thus fill in the left hand of the integer with 1’s as it was shifted right.

> newi and newu:

These functions input some string of bits into the middle of a given word and
return a new, modified word. This is done by first clearing out the bits in the word where
the relevant bits will be inserted. Some number of 1’s (equal to the width) are shifted to
that position in the word using the lsb. This string is inverted so there are zeros in that
position instead and 1’s elsewhere. This string is &’d with the word so all the values are
copied over except for where the bits will be inserted. Then the relevant bits are shifted
over to where they are to be inserted (again using the lsb) and an or operator is used to
copy the modified word with the relevant bits. The result is returned wrapped in an
option. If the value specified does not fit into the width specified, these bit shifts do not
occur and a None option is returned.
