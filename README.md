# Projec P²  
Projec P² is a CLI real projective plane ($\\mathbb{R} P^2$) grapher.

## Description
The purpose of this program is to provide a visualization of the real projective plane, by using the stereographic projection of a [half sphere](https://en.wikipedia.org/wiki/Real_projective_plane#The_projective_hemisphere). There is a detailed explanation on
[this wiki section](https://en.wikipedia.org/wiki/Stereographic_projection#Visualization_of_lines_and_planes).

The [line at infinity](https://en.wikipedia.org/wiki/Line_at_infinity) is fixed on the last coordinate, i.e. $\\{ [x:y:z] | z=0 \\}$.

## Features
The geometrical objects it can draw are:
* `pt`: a point with homogeneous coordinates $[ a : b : c ]$, where `a b c` would be input after `pt`.
* `ln`: a line, determined by 2 points.
* `eq`: a line, given by the coefficients of its equation. Given `a b c`, the line is $\\{ [x:y:z] | ax+by+cz=0 \\}$. It can also be interpreted as the dual hyperplane of $[ a : b : c ]$.
* `cn`: a conic, given a 3x3 matrix. For a correct use, it should be symmetrical; although any 3x3 matrix can be input & drawn.

Moreover, the `help` command prints a quick-glance version of the above list. It can be entered whenever during step 2 (see **Usage**).

## Installation
Download the binary which suits your operating system & run it from the command line. Otherwise, þou canst build it from source.

## Usage
The runtime of the program follows as:
1. Set the size of the output image. It expects only 1 number since it is a square image.
2. Enter geometrical objects (see **Features**) indefinitely until step 3. The `help` command can also be executed here.
3. The `draw` command sets the end of the execution. It expects the name of the output image, which must be `.png`.

## Example
There is an example of a possible input in `input_example.txt`.
The interaction following the example should look like:

```
size of image?
> 400
OK
> ln
1 1 1
-1 0 1
> ln
-1 0 1
0 2 1
> ln
0 2 1
1 1 1 
> pt    
0 1 1
> cn
1 2 3
2 0 -4
3 -4 -1
> draw
output_example.png
END
```

Afterwards, an image `output_example.png` should be created in the same directory from which the program has been run.

![](https://github.com/papanumba/projec_p2/blob/main/output_example.png)
