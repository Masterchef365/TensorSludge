use anyhow::Result;
use mnist::MnistBuilder;
use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use tensorsludge::*;

// Output size is # of rows
fn random_weights(
    mat: Matrix,
    size: usize,
    ts: &mut TensorSludge,
    rng: &mut impl Rng,
) -> Result<()> {
    let unif = Uniform::new(-1., 1.);
    let buf = unif.sample_iter(rng).take(size).collect::<Vec<f32>>();
    ts.write(mat, &buf)
}

/* Wishlist:
 * Sigmoid deriv
 * Elementwise mul
 */

fn main() -> Result<()> {
    let mut ts = TensorSludge::new()?;
    let mnist = MnistBuilder::new().download_and_extract().finalize();

    const IMG_WIDTH: usize = 28;
    const IMG_SIZE: usize = IMG_WIDTH * IMG_WIDTH;
    const HIDDEN_L1: usize = 128;
    const HIDDEN_L2: usize = 64;
    const OUTPUT_SIZE: usize = 10;

    let input_layer = ts.matrix(IMG_SIZE, 1)?;
    let weights_l0 = ts.matrix(HIDDEN_L1, IMG_SIZE)?;
    let activations_l0 = ts.matrix(HIDDEN_L1, 1)?;
    let weights_l1 = ts.matrix(HIDDEN_L2, HIDDEN_L1)?;
    let activations_l1 = ts.matrix(HIDDEN_L2, 1)?;
    let weights_l2 = ts.matrix(OUTPUT_SIZE, HIDDEN_L2)?;
    let output_layer = ts.matrix(OUTPUT_SIZE, 1)?;

    let mut rng = rand::thread_rng();
    random_weights(weights_l0, HIDDEN_L1 * IMG_SIZE, &mut ts, &mut rng)?;
    random_weights(weights_l1, HIDDEN_L2 * HIDDEN_L1, &mut ts, &mut rng)?;
    random_weights(weights_l2, OUTPUT_SIZE * HIDDEN_L2, &mut ts, &mut rng)?;

    let forward_pass = vec![ // The boof
        Operation::MatrixMultiply {
            left: weights_l0,
            right: input_layer,
            dst: activations_l0,
            left_transpose: false,
            right_transpose: false,
        },
        Operation::Sigmoid(activations_l0),
        Operation::MatrixMultiply {
            left: weights_l1,
            right: activations_l0,
            dst: activations_l1,
            left_transpose: false,
            right_transpose: false,
        },
        Operation::Sigmoid(activations_l1),
        Operation::MatrixMultiply {
            left: weights_l2,
            right: activations_l1,
            dst: output_layer,
            left_transpose: false,
            right_transpose: false,
        },
    ];
    let forward_pass = ts.create_pass(&forward_pass)?;

    let mut input_buf = vec![0.; IMG_SIZE];
    let mut output_buf = vec![0.; OUTPUT_SIZE];
    for (label, img) in mnist
        .trn_lbl
        .iter()
        .zip(mnist.trn_img.chunks_exact(IMG_SIZE))
    {
        // Feed forward
        input_buf
            .iter_mut()
            .zip(img.iter().map(|&v| v as f32 / 255.)) // Image normalization
            .for_each(|(o, i)| *o = i);
        ts.write(input_layer, &input_buf)?;
        ts.flow(forward_pass)?;
        ts.read(output_layer, &mut output_buf)?;
        dbg!(&output_buf);
    }

    Ok(())
}
