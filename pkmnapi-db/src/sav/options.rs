use crate::error::Result;
use crate::patch::*;
use crate::sav::Sav;

impl Sav {
    /// Get save options
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::sav::*;
    /// use std::fs;
    /// # use std::env;
    /// # let sav_path = env::var("PKMN_SAV").expect("Set the PKMN_SAV environment variable to point to the SAV location");
    ///
    /// let sav_data = fs::read(sav_path).unwrap();
    /// let sav = Sav::new(&sav_data).unwrap();
    ///
    /// let options = sav.get_options().unwrap();
    ///
    /// assert_eq!(
    ///     options,
    ///     SaveOptions {
    ///         text_speed: SaveTextSpeed::FAST,
    ///         battle_animation: SaveBattleAnimation::ON,
    ///         battle_style: SaveBattleStyle::SHIFT,
    ///     }
    /// );
    /// ```
    pub fn get_options(&self) -> Result<SaveOptions> {
        let offset = 0x2601;

        let save_options = SaveOptions::from(&self.sav[offset]);

        Ok(save_options)
    }

    /// Set save options
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
    /// use pkmnapi_db::sav::*;
    /// use std::fs;
    /// # use std::env;
    /// # let sav_path = env::var("PKMN_SAV").expect("Set the PKMN_SAV environment variable to point to the SAV location");
    ///
    /// let sav_data = fs::read(sav_path).unwrap();
    /// let sav = Sav::new(&sav_data).unwrap();
    ///
    /// let patch = sav
    ///     .set_options(
    ///         &SaveOptions {
    ///             text_speed: SaveTextSpeed::SLOW,
    ///             battle_animation: SaveBattleAnimation::OFF,
    ///             battle_style: SaveBattleStyle::SET
    ///         }
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x2601,
    ///         length: 0x01,
    ///         data: vec![0xC5]
    ///     }
    /// );
    /// ```
    pub fn set_options(&self, save_options: &SaveOptions) -> Result<Patch> {
        let offset = 0x2601;

        Ok(Patch::new(&offset, &save_options.to_raw()))
    }
}

/// Save options
///
/// # Example
///
/// ```
/// use pkmnapi_db::sav::*;
///
/// let save_options = SaveOptions::from(&0xC5);
///
/// assert_eq!(
///     save_options,
///     SaveOptions {
///         text_speed: SaveTextSpeed::SLOW,
///         battle_animation: SaveBattleAnimation::OFF,
///         battle_style: SaveBattleStyle::SET
///     }
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct SaveOptions {
    pub text_speed: SaveTextSpeed,
    pub battle_animation: SaveBattleAnimation,
    pub battle_style: SaveBattleStyle,
}

impl From<&u8> for SaveOptions {
    /// Convert &u8 to save options
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::sav::*;
    ///
    /// let save_options = SaveOptions::from(&0xC5);
    ///
    /// assert_eq!(
    ///     save_options,
    ///     SaveOptions {
    ///         text_speed: SaveTextSpeed::SLOW,
    ///         battle_animation: SaveBattleAnimation::OFF,
    ///         battle_style: SaveBattleStyle::SET
    ///     }
    /// );
    /// ```
    fn from(sav: &u8) -> Self {
        SaveOptions {
            text_speed: SaveTextSpeed::from(&(sav & 0x0F)),
            battle_animation: SaveBattleAnimation::from(&((sav & 0x40) >> 6)),
            battle_style: SaveBattleStyle::from(&((sav & 0x80) >> 7)),
        }
    }
}

impl SaveOptions {
    pub fn to_raw(&self) -> Vec<u8> {
        let value = {
            self.text_speed.value()
                | (self.battle_animation.value() << 6)
                | (self.battle_style.value() << 7)
        };

        vec![value]
    }
}

/// Save text speed
///
/// ```
/// use pkmnapi_db::sav::*;
///
/// let save_text_speed_slow = SaveTextSpeed::SLOW;
/// let save_text_speed_medium = SaveTextSpeed::MEDIUM;
/// let save_text_speed_fast = SaveTextSpeed::FAST;
/// ```
#[derive(Debug, PartialEq)]
pub enum SaveTextSpeed {
    SLOW,
    MEDIUM,
    FAST,
}

impl From<&u8> for SaveTextSpeed {
    /// Convert &u8 to SaveTextSpeed
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::sav::*;
    ///
    /// let save_text_speed = SaveTextSpeed::from(&0x01);
    ///
    /// assert_eq!(save_text_speed, SaveTextSpeed::FAST);
    ///
    /// let save_text_speed = SaveTextSpeed::from(&0x03);
    ///
    /// assert_eq!(save_text_speed, SaveTextSpeed::MEDIUM);
    ///
    /// let save_text_speed = SaveTextSpeed::from(&0x05);
    ///
    /// assert_eq!(save_text_speed, SaveTextSpeed::SLOW);
    /// ```
    fn from(save_text_speed: &u8) -> Self {
        match save_text_speed {
            0x01 => SaveTextSpeed::FAST,
            0x05 => SaveTextSpeed::SLOW,
            _ => SaveTextSpeed::MEDIUM,
        }
    }
}

impl SaveTextSpeed {
    /// Save text speed to value
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::sav::*;
    ///
    /// let save_text_speed_fast = SaveTextSpeed::FAST;
    ///
    /// assert_eq!(save_text_speed_fast.value(), 0x01);
    ///
    /// let save_text_speed_medium = SaveTextSpeed::MEDIUM;
    ///
    /// assert_eq!(save_text_speed_medium.value(), 0x03);
    ///
    /// let save_text_speed_slow = SaveTextSpeed::SLOW;
    ///
    /// assert_eq!(save_text_speed_slow.value(), 0x05);
    /// ```
    pub fn value(&self) -> u8 {
        match self {
            SaveTextSpeed::FAST => 0x01,
            SaveTextSpeed::MEDIUM => 0x03,
            SaveTextSpeed::SLOW => 0x05,
        }
    }
}

/// Save battle animation
///
/// ```
/// use pkmnapi_db::sav::*;
///
/// let save_battle_animation_on = SaveBattleAnimation::ON;
/// let save_battle_animation_off = SaveBattleAnimation::OFF;
/// ```
#[derive(Debug, PartialEq)]
pub enum SaveBattleAnimation {
    ON,
    OFF,
}

impl From<&u8> for SaveBattleAnimation {
    /// Convert &u8 to SaveBattleAnimation
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::sav::*;
    ///
    /// let save_battle_animation = SaveBattleAnimation::from(&0x00);
    ///
    /// assert_eq!(save_battle_animation, SaveBattleAnimation::ON);
    ///
    /// let save_battle_animation = SaveBattleAnimation::from(&0x01);
    ///
    /// assert_eq!(save_battle_animation, SaveBattleAnimation::OFF);
    /// ```
    fn from(save_battle_animation: &u8) -> Self {
        match save_battle_animation {
            0x01 => SaveBattleAnimation::OFF,
            _ => SaveBattleAnimation::ON,
        }
    }
}

impl SaveBattleAnimation {
    /// Save battle animation to value
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::sav::*;
    ///
    /// let save_battle_animation_on = SaveBattleAnimation::ON;
    ///
    /// assert_eq!(save_battle_animation_on.value(), 0x00);
    ///
    /// let save_battle_animation_off = SaveBattleAnimation::OFF;
    ///
    /// assert_eq!(save_battle_animation_off.value(), 0x01);
    /// ```
    pub fn value(&self) -> u8 {
        match self {
            SaveBattleAnimation::ON => 0x00,
            SaveBattleAnimation::OFF => 0x01,
        }
    }
}

/// Save battle style
///
/// ```
/// use pkmnapi_db::sav::*;
///
/// let save_battle_style_shift = SaveBattleStyle::SHIFT;
/// let save_battle_style_set = SaveBattleStyle::SET;
/// ```
#[derive(Debug, PartialEq)]
pub enum SaveBattleStyle {
    SHIFT,
    SET,
}

impl From<&u8> for SaveBattleStyle {
    /// Convert &u8 to SaveBattleStyle
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::sav::*;
    ///
    /// let save_battle_style = SaveBattleStyle::from(&0x00);
    ///
    /// assert_eq!(save_battle_style, SaveBattleStyle::SHIFT);
    ///
    /// let save_battle_style = SaveBattleStyle::from(&0x01);
    ///
    /// assert_eq!(save_battle_style, SaveBattleStyle::SET);
    /// ```
    fn from(save_battle_style: &u8) -> Self {
        match save_battle_style {
            0x01 => SaveBattleStyle::SET,
            _ => SaveBattleStyle::SHIFT,
        }
    }
}

impl SaveBattleStyle {
    /// Save battle style to value
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::sav::*;
    ///
    /// let save_battle_style_shift = SaveBattleStyle::SHIFT;
    ///
    /// assert_eq!(save_battle_style_shift.value(), 0x00);
    ///
    /// let save_battle_style_set = SaveBattleStyle::SET;
    ///
    /// assert_eq!(save_battle_style_set.value(), 0x01);
    /// ```
    pub fn value(&self) -> u8 {
        match self {
            SaveBattleStyle::SHIFT => 0x00,
            SaveBattleStyle::SET => 0x01,
        }
    }
}
