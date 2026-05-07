use global_hotkey::hotkey::HotKey;
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState};
use iced::futures::sink::SinkExt;
use iced::{stream, Subscription};
use std::str::FromStr;

// Fallback constants since the original crate::constants weren't provided
const DEFAULT_HOTKEY: &str = "Control+Shift+Space";
const ERROR_HOTKEY_PARSE: &str = "Failed to parse hotkey combination";
const ERROR_HOTKEY_REGISTER: &str = "Failed to register hotkey with the system";
const STATUS_HOTKEY_REGISTERED: &str = "Hotkey active";
const STATUS_HOTKEY_FAILED: &str = "Hotkey registration failed";
const STATUS_READY: &str = "Ready";

pub struct HotkeyManager {
    manager: GlobalHotKeyManager,
    current_hotkey: Option<HotKey>,
    enabled: bool,
    combination: String,
}

impl HotkeyManager {
    pub fn new() -> Self {
        // Initialize the manager once. 
        // Note: GlobalHotKeyManager is a handle to a singleton-like system underlying it.
        Self {
            manager: GlobalHotKeyManager::new().expect("Failed to initialize Global HotKey Manager"),
            current_hotkey: None,
            enabled: false,
            combination: DEFAULT_HOTKEY.to_string(),
        }
    }

    pub fn with_config(combination: String, enabled: bool) -> Self {
        let mut manager = Self::new();
        manager.combination = combination;
        manager.enabled = enabled;

        if let Ok(hotkey) = Self::parse_hotkey(&manager.combination) {
            manager.current_hotkey = Some(hotkey);
            if enabled {
                let _ = manager.register_current_hotkey();
            }
        }

        manager
    }

    pub fn parse_hotkey(combination: &str) -> Result<HotKey, String> {
        HotKey::from_str(combination).map_err(|_| ERROR_HOTKEY_PARSE.to_string())
    }

    pub fn set_enabled(&mut self, enabled: bool) -> Result<String, String> {
        if enabled == self.enabled {
            return Ok(self.get_status_message());
        }

        if enabled {
            self.enabled = true;
            if self.register_current_hotkey() {
                Ok(STATUS_HOTKEY_REGISTERED.to_string())
            } else {
                self.enabled = false;
                Err(STATUS_HOTKEY_FAILED.to_string())
            }
        } else {
            self.unregister_current_hotkey();
            self.enabled = false;
            Ok("Hotkeys disabled".to_string())
        }
    }

    pub fn update_combination(&mut self, new_combination: String) -> Result<String, String> {
        let parsed_hotkey = Self::parse_hotkey(&new_combination)?;

        // Unregister old key before updating
        if self.enabled {
            self.unregister_current_hotkey();
        }

        self.combination = new_combination;
        self.current_hotkey = Some(parsed_hotkey);

        if self.enabled {
            if self.register_current_hotkey() {
                Ok(STATUS_HOTKEY_REGISTERED.to_string())
            } else {
                Err(ERROR_HOTKEY_REGISTER.to_string())
            }
        } else {
            Ok("Hotkey updated (disabled)".to_string())
        }
    }

    fn register_current_hotkey(&self) -> bool {
        if let Some(hotkey) = self.current_hotkey {
            return self.manager.register(hotkey).is_ok();
        }
        false
    }

    fn unregister_current_hotkey(&self) -> bool {
        if let Some(hotkey) = self.current_hotkey {
            return self.manager.unregister(hotkey).is_ok();
        }
        false
    }

    pub fn get_status_message(&self) -> String {
        if self.enabled {
            if self.current_hotkey.is_some() {
                STATUS_HOTKEY_REGISTERED.to_string()
            } else {
                STATUS_HOTKEY_FAILED.to_string()
            }
        } else {
            STATUS_READY.to_string()
        }
    }

    pub fn create_subscription<T>(&self) -> Subscription<T>
    where
        T: 'static + Send + Clone + From<HotkeyEvent>,
    {
        if !self.enabled {
            return Subscription::none();
        }

        // Capture the ID of the registered hotkey to filter events
        let target_id = self.current_hotkey.map(|h| h.id());

        if let Some(hotkey_id) = target_id {
            // We use iced::stream::channel to create a reactive bridge
            return Subscription::run(move || {
                stream::channel(10, move |mut output| async move {
                    let receiver = GlobalHotKeyEvent::receiver();
                    
                    loop {
                        // try_recv is used here because global-hotkey doesn't provide an async receiver yet
                        // but we reduce the poll rate or use yield to keep the CPU usage low
                        if let Ok(event) = receiver.try_recv() {
                            // IMPORTANT: Filter by ID so we only react to OUR hotkey
                            if event.id == hotkey_id {
                                match event.state() {
                                    HotKeyState::Pressed => {
                                        let _ = output.send(T::from(HotkeyEvent::Pressed)).await;
                                    }
                                    HotKeyState::Released => {
                                        let _ = output.send(T::from(HotkeyEvent::Released)).await;
                                    }
                                }
                            }
                        }
                        
                        // Yielding to the executor to prevent 100% CPU usage
                        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                    }
                })
            });
        }

        Subscription::none()
    }
}

impl Default for HotkeyManager {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub enum HotkeyEvent {
    Pressed,
    Released,
}