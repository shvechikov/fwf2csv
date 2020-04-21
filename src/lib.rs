use std::fs::File;
use std::io::{self, BufReader, BufWriter, Stdout};

use bstr::{ByteSlice, io::BufReadExt};
use csv;

const FIELDS: &'static [(usize, usize)] = &[
    (0, 9), (9, 15), (15, 16), (16, 17), (17, 18), (18, 19), (19, 20), (20, 21), (21, 22), (22, 23), (23, 24), (24, 25),
    (25, 26), (26, 27), (27, 28), (28, 29), (29, 30), (30, 31), (31, 32), (32, 152), (152, 216), (216, 280), (280, 330),
    (330, 343), (343, 349), (349, 399), (399, 402), (402, 452), (452, 462), (462, 465), (465, 470), (470, 474), (474, 476),
    (476, 481), (481, 531), (531, 534), (534, 535), (535, 599), (599, 663), (663, 713), (713, 726), (726, 732), (732, 782),
    (782, 785), (785, 835), (835, 845), (845, 848), (848, 853), (853, 857), (857, 859), (859, 864), (864, 914), (914, 917),
    (917, 918), (918, 927), (927, 1047), (1047, 1111), (1111, 1175), (1175, 1239), (1239, 1245), (1245, 1295), (1295, 1305),
    (1305, 1310), (1310, 1314), (1314, 1316), (1316, 1321), (1321, 1324), (1324, 1333), (1333, 1453), (1453, 1517), (1517, 1581),
    (1581, 1645), (1645, 1651), (1651, 1654), (1654, 1704), (1704, 1714), (1714, 1719), (1719, 1723), (1723, 1725), (1725, 1730),
    (1730, 1780), (1780, 1783), (1783, 1784), (1784, 1793), (1793, 1913), (1913, 1977), (1977, 2041), (2041, 2105), (2105, 2111),
    (2111, 2114), (2114, 2164), (2164, 2174), (2174, 2179), (2179, 2183), (2183, 2185), (2185, 2190), (2190, 2240), (2240, 2243),
    (2243, 2244), (2244, 2246), (2246, 2255), (2255, 2259), (2259, 2379), (2379, 2499), (2499, 2619), (2619, 2739), (2739, 2859),
    (2859, 2979), (2979, 2983), (2983, 2999), (2999, 3015), (3015, 3031), (3031, 3035), (3035, 3036), (3036, 3037), (3037, 3052),
    (3052, 3053), (3053, 3071), (3071, 3072), (3072, 3088), (3088, 3089), (3089, 3105), (3105, 3106), (3106, 3122), (3122, 3128),
    (3128, 3134), (3134, 3138), (3138, 3139), (3139, 3149), (3149, 3150), (3150, 3160), (3160, 3161), (3161, 3171), (3171, 3177),
    (3177, 3183), (3183, 3184), (3184, 3194), (3194, 3195), (3195, 3205), (3205, 3213), (3213, 3233), (3233, 3253), (3253, 3273),
    (3273, 3293), (3293, 3313), (3313, 3333), (3333, 3393), (3393, 3453), (3453, 3458), (3458, 3468), (3468, 3481), (3481, 3482),
    (3482, 3497), (3497, 3500), (3500, 3508), (3508, 3516), (3516, 3520), (3520, 3528), (3528, 3529), (3529, 3532), (3532, 3533),
    (3533, 3536), (3536, 3537), (3537, 3538), (3538, 3539), (3539, 3542), (3542, 3583), (3583, 3584), (3584, 3585), (3585, 3586),
    (3586, 3587), (3587, 3588), (3588, 3589), (3589, 3590), (3590, 3591), (3591, 3600), (3600, 3630), (3630, 3655), (3655, 3675),
    (3675, 3677), (3677, 3682), (3682, 3686), (3686, 3688), (3688, 3693), (3693, 3694), (3694, 3700), (3700, 3706), (3706, 3714),
    (3714, 3722), (3722, 3724), (3724, 3727), (3727, 3731), (3731, 3752), (3752, 3756), (3756, 3759), (3759, 3761), (3761, 3764),
    (3764, 3766), (3766, 3796), (3796, 3799), (3799, 3803), (3803, 3812), (3812, 3818), (3818, 3820), (3820, 3821), (3821, 3841),
    (3841, 3846), (3846, 3847), (3847, 3848), (3848, 3849), (3849, 3850), (3850, 3860), (3860, 3861), (3861, 3863), (3863, 3864),
    (3864, 3865), (3865, 3866), (3866, 3867), (3867, 3868), (3868, 3869), (3869, 3870), (3870, 3871), (3871, 3881), (3881, 3882),
    (3882, 3883), (3883, 3884), (3884, 3885), (3885, 3886), (3886, 3887), (3887, 3888), (3888, 3889), (3889, 3890), (3890, 3891),
    (3891, 3892), (3892, 3893), (3893, 3894), (3894, 3895), (3895, 3896), (3896, 3897), (3897, 3898), (3898, 3899), (3899, 3900),
    (3900, 3901), (3901, 3902), (3902, 3903), (3903, 3904), (3904, 3905), (3905, 3906), (3906, 3908), (3908, 3910), (3910, 3913),
    (3913, 3961), (3961, 3963), (3963, 3965), (3965, 3967), (3967, 3968), (3968, 3978), (3978, 3989), (3989, 3990), (3990, 3991),
    (3991, 3992),
];

fn slice<'a>(line: &'a [u8]) -> impl Iterator<Item = &[u8]> + 'a {
    FIELDS
        .iter()
        .map(move |&(start, end)| line[start..end].trim_end())
}

#[inline]
pub fn parse_gdmi(
    reader: &mut BufReader<File>,
    writer: &mut csv::Writer<Stdout>,
) -> io::Result<()> {
    reader.for_byte_line(|line| {
        // 1:
        // let mut record: [&[u8]; FIELDS_NUMBER] = [b""; FIELDS_NUMBER];
        // for (i, &(start, end)) in FIELDS.iter().enumerate() {
        //     if i >= FIELDS_NUMBER { break };
        //     record[i] = &line[start..end].trim_end();
        // };

        // 2:
        // let record: Vec<&[u8]> = FIELDS
        //     .iter()
        //     .map(|&(start, end)| { line[start..end].trim_end() })
        //     .collect();

        // 3:
        let lazy_record = slice(&line);

        writer.write_record(lazy_record)?;

        writer.flush()?;
        Ok(true)
    })
}
