# Projec P²
Projec P² is a real projective plane ($\\mathbb{R} P^2$) grapher.

## Description
The purpose of this program is to provide a visualization of the real projective plane, by using the stereographic projection of a [half sphere](https://en.wikipedia.org/wiki/Real_projective_plane#The_projective_hemisphere). There is a detailed explanation on
[this wiki section](https://en.wikipedia.org/wiki/Stereographic_projection#Visualization_of_lines_and_planes).

The [line at infinity](https://en.wikipedia.org/wiki/Line_at_infinity) is fixed on the last coordinate, i.e. $\\{ [x:y:z] | z=0 \\}$.

## Features
The geometrical objects it can draw are:
* `pt`: a point with homogeneous coordinates $[ a : b : c ]$.
* `ln`: a line, determined by 2 points.
* `eq`: a line, given by the coefficients of its equation. Given `a b c`, the line is $\\{ [x:y:z] | ax+by+cz=0 \\}$. It can also be interpreted as the dual hyperplane of $[ a : b : c ]$.
* `cn`: a conic, given a 3x3 matrix. For a correct use, it should be symmetrical; although any 3x3 matrix can be input & drawn.

## Installation
TODO

## Dependencies
python3: Pillow >= 8.4, & manual installation of ImageTk

## Usage
Select a figure from the radiobuttons & click `NEW FIG` to create a new entry.
Enter the numbers/coordinates (empty entries are by default `0.0`).
Then, click `DRAW` to draw the entered figures. The icon under `DRAW` will show
whether there is a syntax error. See the underlying terminal to read the error message.
If you want to delete a figure, press its red `X` button.

To use parameters, enter its name in the top entry and click `NEW PARAM`.
Then the entered string can be used as a variable parameter in the figures.

## Example
![](https://github.com/papanumba/projec_p2/blob/py-gui/scrot0.png)
