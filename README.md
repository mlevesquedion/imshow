# imshow

A command line tool to display images in the terminal.

The terminal must support ANSI RGB sequences.

## TODO
* Use half-blocks instead of full-blocks
* Improve performance (profile with criterion first; slow on large images; use Iterators everywhere + rayon?)
* Encapsulate terminal cell size handling (about twice as high as wide)
* Introduce property-based tests where appropriate (lengths, normalized\_by\_sum)
* Take alpha channel into account

## Similar Projects
* [viu](https://github.com/atanunq/viu)
* [termimage](https://github.com/nabijaczleweli/termimage)
* [termpix](https://github.com/hopey-dishwasher/termpix)

## Trivia

The name is a reference to Matlab's imshow, which I know through matplotlib's (Python) imshow.
