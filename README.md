# astrup

A rust plotting library.

## Gloals:
- Input `Vec<Num>` and `ndarray Array*` types
- It should be intuitive to build plots, but not as "easy as possible". It will probably be
quite verbose.
- Very modular.


## Structurs

### Figure

The main image window. Can contain multiple plots.

### Plot

An area defined by an x and y axis. Can contain multiple drawable objects

### Drawable objects

These methods draw whatever they specify onto its plot. It
should be possible to combine as many as you want of any combination.

The variants currently intended are

| Variant       | Supported |
| ------------- | --------- |
| Scatter       | Partially |
| Line          | Parially  |
| Histogram     | No        |
| BoxPlot       | No        |
| MatrixHeatmap | No        |
| Image         | No        |

## TODO:

### Major
- One window for each figure
- Multiple plots (arranged in a grid) in one figure
- Implement the above plot variants

### Minor
- Refactor fit and scaling functions
- Axis label and tick label sizes seems to scale differently
