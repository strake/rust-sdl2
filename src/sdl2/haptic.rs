//! Haptic Functions
use sys;

use HapticSubsystem;
use common::IntegerOrSdlError;
use get_error;

impl HapticSubsystem {
    /// Attempt to open the joystick at number `id` and return it.
    pub fn open_from_joystick_id(&self, joystick_index: i32) -> Result<Haptic, IntegerOrSdlError> {
        use common::IntegerOrSdlError::*;

        let haptic = unsafe {
            let joystick = sys::SDL_JoystickOpen(joystick_index);
            sys::SDL_HapticOpenFromJoystick(joystick)
        };

        if haptic.is_null() {
            Err(SdlError(get_error()))
        } else {
            unsafe { sys::SDL_HapticRumbleInit(haptic) };
            Ok(Haptic {
                subsystem: self.clone(),
                raw: haptic,
            })
        }
    }
}

/// Wrapper around the `SDL_Haptic` object
pub struct Haptic {
    subsystem: HapticSubsystem,
    raw: *mut sys::SDL_Haptic,
}


impl Haptic {
    #[inline]
    pub fn subsystem(&self) -> &HapticSubsystem { &self.subsystem }

    /// Run a simple rumble effect on the haptic device.
    pub fn rumble_play(&mut self, strength: f32, duration: u32) {
        unsafe { sys::SDL_HapticRumblePlay(self.raw, strength, duration) };
    }

    /// Stop the simple rumble on the haptic device.
    pub fn rumble_stop(&mut self) {
        unsafe { sys::SDL_HapticRumbleStop(self.raw) };
    }
}


impl Drop for Haptic {
    fn drop(&mut self) {
        unsafe { sys::SDL_HapticClose(self.raw) }
    }
}
