use rm::linalg::Matrix;
use rm::learning::UnSupModel;
use rm::learning::pca::PCA;

#[test]
fn test_default() {
    let mut model = PCA::default();

    let inputs = Matrix::new(7, 3, vec![8.3, 50., 23.,
                                        10.2, 55., 21.,
                                        11.1, 57., 22.,
                                        12.5, 60., 15.,
                                        11.3, 59., 20.,
                                        12.4, 61., 11.,
                                        11.2, 58., 23.]);

    model.train(&inputs).unwrap();

    let cexp = Matrix::new(3, 3, vec![0.2304196717022202, 0.2504639278931734, -0.9403055863478447,
                                      0.5897383434061588, 0.7326863014098074, 0.3396755364211204,
                                      -0.7740254913174374, 0.6328021843757651, -0.021117155112842168]);
    let diff = model.components().unwrap() - cexp;
    assert_eq!(diff.into_vec().iter().all(|&x| x < 1e-8), true);

    let new_data = Matrix::new(1, 3, vec![9., 45., 22.]);
    let outputs = model.predict(&new_data).unwrap();

    let exp = Matrix::new(1, 3, vec![-9.72287413262656, -7.680227015314077, -2.301338333438487]);
    let diff = outputs - exp;
    assert_eq!(diff.into_vec().iter().all(|&x| x < 1e-8), true);
}

#[test]
fn test_not_center() {
    let mut model = PCA::new(false);

    let inputs = Matrix::new(7, 3, vec![8.3, 50., 23.,
                                        10.2, 55., 21.,
                                        11.1, 57., 22.,
                                        12.5, 60., 15.,
                                        11.3, 59., 20.,
                                        12.4, 61., 11.,
                                        11.2, 58., 23.]);

    model.train(&inputs).unwrap();

    let cexp = Matrix::new(3, 3, vec![0.17994480617740657, -0.16908609066166264, 0.9690354795746806,
                                      0.9326216647416523, -0.2839205184846983, -0.2227239763426676,
                                      0.3127885822473139, 0.9438215049087068, 0.10660332868901998]);
    let diff = model.components().unwrap() - cexp;
    assert_eq!(diff.into_vec().iter().all(|&x| x < 1e-8), true);

    let new_data = Matrix::new(1, 3, vec![9., 45., 22.]);
    let outputs = model.predict(&new_data).unwrap();

    let exp = Matrix::new(1, 3, vec![50.468826978411926, 6.465874960225161, 1.0440136119105228]);
    let diff = outputs - exp;
    assert_eq!(diff.into_vec().iter().all(|&x| x < 1e-8), true);
}