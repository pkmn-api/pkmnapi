/// New licensee code
///
/// # Example
///
/// ```
/// use pkmnapi_db::header::*;
///
/// let code = NewLicenseeCode::NINTENDO_RND1;
/// ```
#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum NewLicenseeCode {
    NONE,
    NINTENDO_RND1,
    CAPCOM,
    ELECTRONIC_ARTS,
    HUDSON_SOFT,
    B_AI,
    KSS,
    POW,
    PCM_COMPLETE,
    SAN_X,
    KEMCO_JAPAN,
    SETA,
    VIACOM,
    NINTENDO,
    BANDAI,
    OCEAN_ACCLAIM,
    KONAMI,
    HECTOR,
    TAITO,
    HUDSON,
    BANPRESTO,
    UBI_SOFT,
    ATLUS,
    MALIBU,
    ANGEL,
    BULLET_PROOF,
    IREM,
    ABSOLUTE,
    ACCLAIM,
    ACTIVISION,
    AMERICAN_SAMMY,
    KONAMI_2,
    HI_TECH_ENTERTAINMENT,
    LJN,
    MATCHBOX,
    MATTEL,
    MILTON_BRADLEY,
    TITUS,
    VIRGIN,
    LUCASARTS,
    OCEAN,
    ELECTRONIC_ARTS_2,
    INFOGRAMES,
    INTERPLAY,
    BRODERBUND,
    SCULPTURED,
    SCI,
    THQ,
    ACCOLADE,
    MISAWA,
    LOZC,
    TOKUMA_SHOTEN_INTERMEDIA,
    TSUKUDA_ORIGINAL,
    CHUNSOFT,
    VIDEO_SYSTEM,
    OCEAN_ACCLAIM_2,
    VARIE,
    YONEZAWA_S_PAL,
    KANEKO,
    PACK_IN_SOFT,
    KONAMI_YU_GI_OH,
}

impl From<Vec<u8>> for NewLicenseeCode {
    /// Convert Vec<u8> to NewLicenseeCode
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::header::*;
    ///
    /// let code = NewLicenseeCode::from(vec![0x30, 0x31]);
    ///
    /// assert_eq!(code, NewLicenseeCode::NINTENDO_RND1);
    /// ```
    fn from(new_licensee_code: Vec<u8>) -> Self {
        let code: String = new_licensee_code.iter().map(|x| *x as char).collect();

        match code.as_str() {
            "01" => NewLicenseeCode::NINTENDO_RND1,
            "08" => NewLicenseeCode::CAPCOM,
            "13" => NewLicenseeCode::ELECTRONIC_ARTS,
            "18" => NewLicenseeCode::HUDSON_SOFT,
            "19" => NewLicenseeCode::B_AI,
            "20" => NewLicenseeCode::KSS,
            "22" => NewLicenseeCode::POW,
            "24" => NewLicenseeCode::PCM_COMPLETE,
            "25" => NewLicenseeCode::SAN_X,
            "28" => NewLicenseeCode::KEMCO_JAPAN,
            "29" => NewLicenseeCode::SETA,
            "30" => NewLicenseeCode::VIACOM,
            "31" => NewLicenseeCode::NINTENDO,
            "32" => NewLicenseeCode::BANDAI,
            "33" => NewLicenseeCode::OCEAN_ACCLAIM,
            "34" => NewLicenseeCode::KONAMI,
            "35" => NewLicenseeCode::HECTOR,
            "37" => NewLicenseeCode::TAITO,
            "38" => NewLicenseeCode::HUDSON,
            "39" => NewLicenseeCode::BANPRESTO,
            "41" => NewLicenseeCode::UBI_SOFT,
            "42" => NewLicenseeCode::ATLUS,
            "44" => NewLicenseeCode::MALIBU,
            "46" => NewLicenseeCode::ANGEL,
            "47" => NewLicenseeCode::BULLET_PROOF,
            "49" => NewLicenseeCode::IREM,
            "50" => NewLicenseeCode::ABSOLUTE,
            "51" => NewLicenseeCode::ACCLAIM,
            "52" => NewLicenseeCode::ACTIVISION,
            "53" => NewLicenseeCode::AMERICAN_SAMMY,
            "54" => NewLicenseeCode::KONAMI_2,
            "55" => NewLicenseeCode::HI_TECH_ENTERTAINMENT,
            "56" => NewLicenseeCode::LJN,
            "57" => NewLicenseeCode::MATCHBOX,
            "58" => NewLicenseeCode::MATTEL,
            "59" => NewLicenseeCode::MILTON_BRADLEY,
            "60" => NewLicenseeCode::TITUS,
            "61" => NewLicenseeCode::VIRGIN,
            "64" => NewLicenseeCode::LUCASARTS,
            "67" => NewLicenseeCode::OCEAN,
            "69" => NewLicenseeCode::ELECTRONIC_ARTS_2,
            "70" => NewLicenseeCode::INFOGRAMES,
            "71" => NewLicenseeCode::INTERPLAY,
            "72" => NewLicenseeCode::BRODERBUND,
            "73" => NewLicenseeCode::SCULPTURED,
            "75" => NewLicenseeCode::SCI,
            "78" => NewLicenseeCode::THQ,
            "79" => NewLicenseeCode::ACCOLADE,
            "80" => NewLicenseeCode::MISAWA,
            "83" => NewLicenseeCode::LOZC,
            "86" => NewLicenseeCode::TOKUMA_SHOTEN_INTERMEDIA,
            "87" => NewLicenseeCode::TSUKUDA_ORIGINAL,
            "91" => NewLicenseeCode::CHUNSOFT,
            "92" => NewLicenseeCode::VIDEO_SYSTEM,
            "93" => NewLicenseeCode::OCEAN_ACCLAIM_2,
            "95" => NewLicenseeCode::VARIE,
            "96" => NewLicenseeCode::YONEZAWA_S_PAL,
            "97" => NewLicenseeCode::KANEKO,
            "99" => NewLicenseeCode::PACK_IN_SOFT,
            "A4" => NewLicenseeCode::KONAMI_YU_GI_OH,
            _ => NewLicenseeCode::NONE,
        }
    }
}
