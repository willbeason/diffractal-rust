# Examples

For now, these are speculative design requirements of generative images this software will support.


## Mandelbrot and Julia Sets

**Possible Inputs**:
- Grid representing set of pixels on image
- Volume of initial "interesting points"

**Possible Transforms**:
- Complex polynomials
- Absolute value
- Logarithm

**Possible Data**:
- Threshold (e.g. number of iterations before absolute value above limit)
- Hits of sub-regions (paths)

## Barnsley's Fern and Similar

**Possible Transforms**
- Affine transforms
- Stochastic matrix

## Differential Equations

**Possible Transforms**:
- System of equations

**Possible Data**:
- End state of transformed volume (e.g. Duffing Oscillator)
- Iterations as frames

## Generative Landscapes

**Possible Transforms**:
- Simulations
- Noise functions (e.g. Perlin)
