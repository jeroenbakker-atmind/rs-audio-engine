use crate::{
    processor::DefaultStringProcessor, processor::StringProcessor, string::CELLO_STRING_G2,
};

#[test]
fn string_processor() {
    let string = CELLO_STRING_G2;
    let mut string_processor = DefaultStringProcessor::new(44100.0, &string);
    string_processor.bow.velocity = 0.2;
    string_processor.bow.pressure = 10.0;
    string_processor.gain = 1000.0;
    for _ in 0..4096 {
        let sample = string_processor.read_output();
        println!("{sample:?}");
    }
}

mod validation {
    //! Validate the first 50 samples to be "same" as the output from mathlab.
    use crate::{
        processor::{modal::ModalProcessor, modal_var1::ModalVar1Processor, DefaultStringProcessor, StringProcessor},
        string::CELLO_STRING_G2,
    };

    static CELLO_G2_SAMPLES: [[f64; 2]; 50] = [
        [-1.7128506630074045e-10, 4.7765708185130249e-10],
        [-2.3169816255290039e-10, 5.0798761840634356e-10],
        [-3.2668100263304052e-11, 6.4051547449242678e-11],
        [-1.1634041899939667e-10, -4.4017066773385328e-10],
        [-2.54988309874056e-10, -5.581068627282698e-09],
        [-7.5868439053087769e-11, -1.320361734661859e-08],
        [-6.5903492594947484e-11, -5.2687741949793808e-09],
        [-2.5031280824036768e-10, 3.1170530464261926e-08],
        [-1.2933855120329836e-10, 8.7360246651643386e-08],
        [-3.1943513653004229e-11, 1.5071852543367516e-07],
        [-2.2139962249432234e-10, 2.2029151747120307e-07],
        [-1.8294347589729992e-10, 2.9622601252935344e-07],
        [-2.059636415096041e-11, 3.738677158648295e-07],
        [-1.7477956870093426e-10, 4.5193643525077289e-07],
        [-2.262609149221693e-10, 5.3202833847150061e-07],
        [-3.3290495036011079e-11, 6.1274078520074322e-07],
        [-1.1954943196879133e-10, 6.9283278247139064e-07],
        [-2.5098649650978771e-10, 7.7358976357687012e-07],
        [-6.7100291974259581e-11, 8.5488273510313654e-07],
        [-6.5988715696030969e-11, 9.3556841972132594e-07],
        [-2.5215364000826828e-10, 1.0163544933332517e-06],
        [-1.1310387556399109e-10, 1.0976777968534024e-06],
        [-1.7194105572316329e-11, 1.1786268523972108e-06],
        [-2.2959601236219588e-10, 1.2593875605360352e-06],
        [-2.0870327618225404e-10, 1.340610502558731e-06],
        [-9.5855163375651498e-11, 1.4217101791485923e-06],
        [-2.0572815350320332e-10, 1.5024904943027351e-06],
        [2.1081630258344574e-10, 1.5835886399886053e-06],
        [1.0380255932470878e-09, 1.6647661053553385e-06],
        [3.7023347734510723e-10, 1.7456106157388263e-06],
        [-2.7975614625831609e-09, 1.8265997070422872e-06],
        [-6.3366876894791854e-09, 1.9077938527010789e-06],
        [-4.8647193797815336e-09, 1.9887255026184858e-06],
        [5.8493585882230848e-09, 2.0696415635711104e-06],
        [1.9787266623641473e-08, 2.1508047494034562e-06],
        [2.2130414942264249e-08, 2.2318223396818575e-06],
        [7.1166783161476154e-09, 2.3127090932815814e-06],
        [-1.4766013354544449e-08, 2.3938123523074964e-06],
        [-3.316055528356633e-08, 2.474895133088325e-06],
        [-4.7253087293939045e-08, 2.5557930880552143e-06],
        [-5.4893578744736526e-08, 2.6368282880768865e-06],
        [-4.9333586596981797e-08, 2.7179442518123626e-06],
        [-2.9400301459812102e-08, 2.7988822424388992e-06],
        [1.1121454399824721e-09, 2.8798598429922864e-06],
        [4.1872620737641718e-08, 2.9609752840279019e-06],
        [9.337789785428936e-08, 3.0419658619211653e-06],
        [1.5232352443884497e-07, 3.1229089247829371e-06],
        [2.1622653186103087e-07, 3.2039969517548624e-06],
        [2.8533635723132687e-07, 3.285036236741956e-06],
        [3.5836562701995532e-07, 3.3659723999453078e-06],
    ];

    fn validate<P>()
    where
        P: StringProcessor + Clone + Default,
    {
        let string = CELLO_STRING_G2;
        let sample_rate = 44100.0;

        let mut processor = P::new(sample_rate, &string);
        processor.update_bow(crate::bow::Bow {
            pressure: 10.0,
            velocity: 0.2,
        });

        for samples in &CELLO_G2_SAMPLES {
            let sum = samples[0] + samples[1];
            let sample = processor.read_output();
            assert!((sum - sample).abs() < 1.0e-20);
        }
    }

    #[test]
    fn validate_default_processor() {
        validate::<DefaultStringProcessor>();
    }

    #[test]
    fn validate_modal_processor() {
        validate::<ModalProcessor>();
    }

    #[test]
    fn validate_modal_var1_processor() {
        validate::<ModalVar1Processor>();
    }
}
