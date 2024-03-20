use crate::transfer_function::TransferFunction;

#[test]
fn test_z() {
    let fs = 44100.0;
    let z = TransferFunction::new(1.0 / fs);
    assert_eq!(1.0 / fs, z.timestep);
    assert_eq!(0.0, z.numerator[0]);
    assert_eq!(1.0, z.numerator[1]);
    assert_eq!(1.0, z.denominator[0]);
    assert_eq!("f(z) = z", z.to_string());
}

#[test]
fn test_dl1() {
    let fs = 44100.0;
    let z = TransferFunction::new(1.0 / fs);
    let m = 38;
    let i0 = 5;
    let dl1 = z.pow(-(m - i0));
    assert_eq!("f(z) = \\frac{1}{z^{33}}", dl1.to_string());
}

#[test]
fn test_dl2() {
    let fs = 44100.0;
    let z = TransferFunction::new(1.0 / fs);
    let i0 = 5;
    let dl1 = z.pow(-i0);
    assert_eq!("f(z) = \\frac{1}{z^5}", dl1.to_string());
}

#[test]
/// Hl=gl*(1+al)/(1+al*z^-1);
fn test_hl() {
    let fs = 44100.0;
    let z = TransferFunction::new(1.0 / fs);
    let gl = -0.99;
    let al = -0.001;
    let result_a = z.pow(-1);
    assert_eq!("f(z) = \\frac{1}{z}", result_a.to_string());
    let result_b = al * result_a;
    assert_eq!("f(z) = \\frac{-0.001}{z}", result_b.to_string());
    let result_c = 1.0 + result_b;
    assert_eq!("f(z) = \\frac{-0.001 +z}{z}", result_c.to_string());
    let result_d = gl * (1.0 + al) / &result_c;
    assert_eq!("f(z) = \\frac{-0.98901*z}{-0.001 +z}", result_d.to_string());

    let result = gl * (1.0 + al) / &(1.0 + al * z.pow(-1));
    assert_eq!("f(z) = \\frac{-0.98901*z}{-0.001 +z}", result.to_string());
}

#[test]
// Hd=(ad+z^-1)/(1+ad*z^-1);
fn test_hd() {
    let fs = 44100.0;
    let z = TransferFunction::new(1.0 / fs);
    let ad = -0.30;

    let lhs = ad + z.pow(-1);
    assert_eq!("f(z) = \\frac{1 -0.3*z}{z}", lhs.to_string());
    let rhs = 1.0 + ad * z.pow(-1);
    assert_eq!("f(z) = \\frac{-0.3 +z}{z}", rhs.to_string());
    let result = lhs / rhs;
    assert_eq!("f(z) = \\frac{z -0.3*z^2}{-0.3*z +z^2}", result.to_string());

    let result = (ad + z.pow(-1)) / (1.0 + ad * z.pow(-1));
    assert_eq!("f(z) = \\frac{z -0.3*z^2}{-0.3*z +z^2}", result.to_string());
}

#[test]
// Hfd1=(C+z^-1)/(1+C*z^-1);
fn test_hfd1() {
    let fs = 44100.0;
    let z = TransferFunction::new(1.0 / fs);
    let c = -0.3242;
    let hfd1 = (c + z.pow(-1)) / (1.0 + c * z.pow(-1));
    assert_eq!(
        "f(z) = \\frac{z -0.3242*z^2}{-0.3242*z +z^2}",
        hfd1.to_string()
    );
}

#[test]
// Hfd2=(C*(1+offtune)+z^-1)/(1+C*(1+offtune)*z^-1);
fn test_hfd2() {
    let fs = 44100.0;
    let z = TransferFunction::new(1.0 / fs);
    let c = -0.3242;
    let offtune = 0.04;
    let hfd2 = (c * (1.0 + offtune) + z.pow(-1)) / (1.0 + c * (1.0 + offtune) * z.pow(-1));
    assert_eq!(
        "f(z) = \\frac{z -0.337168*z^2}{-0.337168*z +z^2}",
        hfd2.to_string()
    );
}

#[test]
// Hfd3=(C*(1-offtune)+z^-1)/(1+C*(1-offtune)*z^-1);
fn test_hfd3() {
    let fs = 44100.0;
    let z = TransferFunction::new(1.0 / fs);
    let c = -0.3242;
    let offtune = 0.04;
    let hfd3 = (c * (1.0 - offtune) + z.pow(-1)) / (1.0 + c * (1.0 - offtune) * z.pow(-1));
    assert_eq!(
        "f(z) = \\frac{z -0.31123199999999995*z^2}{-0.31123199999999995*z +z^2}",
        hfd3.to_string()
    );
}

#[test]
fn test_hd_pow() {
    let fs = 44100.0;
    let z = TransferFunction::new(1.0 / fs);
    let ad = -0.30;

    let hd = (ad + z.pow(-1)) / (1.0 + ad * z.pow(-1));
    assert_eq!("f(z) = \\frac{z -0.3*z^2}{-0.3*z +z^2}", hd.to_string());
    assert_eq!(
        "f(z) = \\frac{z -0.3*z^2}{-0.3*z +z^2}",
        hd.pow(1).to_string()
    );
    assert_eq!(
        "f(z) = \\frac{z^2 -0.6*z^3 +0.09*z^4}{0.09*z^2 -0.6*z^3 +z^4}",
        hd.pow(2).to_string()
    );
}

#[test]
// H1=Hl*Hd^ap_num*Hfd1;
fn test_h1() {
    let fs = 44100.0;
    let z = TransferFunction::new(1.0 / fs);
    let c = -0.3242;
    let ad = -0.30;
    let gl = -0.99;
    let al = -0.001;
    let ap_num = 12;

    let hl = gl * (1.0 + al) / &(1.0 + al * z.pow(-1));
    let hd = (ad + z.pow(-1)) / (1.0 + ad * z.pow(-1));
    let hfd1 = (c + z.pow(-1)) / (1.0 + c * z.pow(-1));
    let h1 = &(&hl * &hd.pow(ap_num)) * &hfd1;
    assert_eq!("f(z) = \\frac{-0.98901*z^{14} +3.881073041999999*z^{15} -7.029012751199999*z^{16} +7.779303429479999*z^{17} -5.8700196244799985*z^{18} +3.189003305498999*z^{19} -1.2832784055115196*z^{20} +0.3872866466470319*z^{21} -0.08765769861913678*z^{22} +0.014695983623781895*z^{23} -0.0017738820973249195*z^{24} +0.0001459837767105828*z^{25} -0.000007341587272910878*z^{26} +0.00000017039967023752195*z^{27}}{0.00000000017229317219999994*z^{13} -0.00000017971634008799996*z^{14} +0.000007570773854279998*z^{15} -0.000149399559972*z^{16} +0.0018084529791899994*z^{17} -0.014947918951679996*z^{18} +0.08902335190319997*z^{19} -0.3928877615519999*z^{20} +1.3007627918999995*z^{21} -3.2303751479999994*z^{22} +5.943113747999998*z^{23} -7.872855119999999*z^{24} +7.1110441999999985*z^{25} -3.925199999999999*z^{26} +z^{27}}", h1.to_string());
}

#[test]
fn test_h2() {
    let fs = 44100.0;
    let z = TransferFunction::new(1.0 / fs);
    let c = -0.3242;
    let ad = -0.30;
    let gl = -0.99;
    let al = -0.001;
    let ap_num = 12;
    let offtune = 0.04;

    let hl = gl * (1.0 + al) / &(1.0 + al * z.pow(-1));
    let hd = (ad + z.pow(-1)) / (1.0 + ad * z.pow(-1));
    let hfd2 = (c * (1.0 + offtune) + z.pow(-1)) / (1.0 + c * (1.0 + offtune) * z.pow(-1));
    let h2 = &(&hl * &hd.pow(ap_num)) * &hfd2;

    assert_eq!("f(z) = \\frac{-0.98901*z^{14} +3.893898523679999*z^{15} -7.075184485247998*z^{16} +7.855486790659199*z^{17} -5.946202985659198*z^{18} +3.240427074294959*z^{19} -1.3079618145335805*z^{20} +0.39592583980475315*z^{21} -0.08987920543112224*z^{22} +0.015112516151029173*z^{23} -0.0018294197676245565*z^{24} +0.0001509821670375501*z^{25} -0.000007614226745290914*z^{26} +0.00000017721565704702284*z^{27}}{0.00000000017918489908799998*z^{13} -0.00000018688373605151997*z^{14} +0.0000078514968628512*z^{15} -0.00015450964783488*z^{16} +0.0018650289519575995*z^{17} -0.015371326231747195*z^{18} +0.09127827956332799*z^{19} -0.4016479121740799*z^{20} +1.3257724811759997*z^{21} -3.2824473739199993*z^{22} +6.020220697919999*z^{23} -7.949931724799999*z^{24} +7.157741967999999*z^{25} -3.9381679999999992*z^{26} +z^{27}}", h2.to_string());
}

#[test]
fn test_h3() {
    let fs = 44100.0;
    let z = TransferFunction::new(1.0 / fs);
    let c = -0.3242;
    let ad = -0.30;
    let gl = -0.99;
    let al = -0.001;
    let ap_num = 12;
    let offtune = 0.04;

    let hl = gl * (1.0 + al) / &(1.0 + al * z.pow(-1));
    let hd = (ad + z.pow(-1)) / (1.0 + ad * z.pow(-1));
    let hfd3 = (c * (1.0 - offtune) + z.pow(-1)) / (1.0 + c * (1.0 - offtune) * z.pow(-1));
    let h3 = &(&hl * &hd.pow(ap_num)) * &hfd3;

    assert_eq!("f(z) = \\frac{-0.98901*z^{14} +3.8682475603199986*z^{15} -6.982841017151999*z^{16} +7.703120068300798*z^{17} -5.793836263300799*z^{18} +3.137579536703039*z^{19} -1.2585949964894587*z^{20} +0.3786474534893106*z^{21} -0.08543619180715128*z^{22} +0.01427945109653462*z^{23} -0.0017183444270252824*z^{24} +0.00014098538638361544*z^{25} -0.000007068947800530842*z^{26} +0.00000016358368342802103*z^{27}}{0.00000000016540144531199993*z^{13} -0.00000017254894412447994*z^{14} +0.000007290050845708798*z^{15} -0.00014428947210911997*z^{16} +0.0017518770064223993*z^{17} -0.014524511671612794*z^{18} +0.08676842424307196*z^{19} -0.38412761092991987*z^{20} +1.2757531026239997*z^{21} -3.1783029220799994*z^{22} +5.866006798079999*z^{23} -7.7957785151999985*z^{24} +7.064346431999999*z^{25} -3.912231999999999*z^{26} +z^{27}}", h3.to_string());
}
