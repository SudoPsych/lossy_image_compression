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

Utilizes bitwise operators to pack bits into binary 'codewords' and extract that data back out.
