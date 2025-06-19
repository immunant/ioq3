use ::libc;

pub use crate::be_aas_h::C2RustUnnamed_0;
pub use crate::stdlib::SDLK_a;
pub use crate::stdlib::SDLK_b;
pub use crate::stdlib::SDLK_c;
pub use crate::stdlib::SDLK_d;
pub use crate::stdlib::SDLK_e;
pub use crate::stdlib::SDLK_f;
pub use crate::stdlib::SDLK_g;
pub use crate::stdlib::SDLK_h;
pub use crate::stdlib::SDLK_i;
pub use crate::stdlib::SDLK_j;
pub use crate::stdlib::SDLK_k;
pub use crate::stdlib::SDLK_l;
pub use crate::stdlib::SDLK_m;
pub use crate::stdlib::SDLK_n;
pub use crate::stdlib::SDLK_o;
pub use crate::stdlib::SDLK_p;
pub use crate::stdlib::SDLK_q;
pub use crate::stdlib::SDLK_r;
pub use crate::stdlib::SDLK_s;
pub use crate::stdlib::SDLK_t;
pub use crate::stdlib::SDLK_u;
pub use crate::stdlib::SDLK_v;
pub use crate::stdlib::SDLK_w;
pub use crate::stdlib::SDLK_x;
pub use crate::stdlib::SDLK_y;
pub use crate::stdlib::SDLK_z;
pub use crate::stdlib::SDL_FingerID;
pub use crate::stdlib::SDL_GameController;
pub use crate::stdlib::SDL_GameControllerAxis;
pub use crate::stdlib::SDL_GameControllerButton;
pub use crate::stdlib::SDL_GameControllerClose;
pub use crate::stdlib::SDL_GameControllerEventState;
pub use crate::stdlib::SDL_GameControllerGetAxis;
pub use crate::stdlib::SDL_GameControllerGetButton;
pub use crate::stdlib::SDL_GameControllerOpen;
pub use crate::stdlib::SDL_GameControllerUpdate;
pub use crate::stdlib::SDL_GestureID;
pub use crate::stdlib::SDL_GetKeyName;
pub use crate::stdlib::SDL_GetScancodeName;
pub use crate::stdlib::SDL_GetWindowFlags;
pub use crate::stdlib::SDL_IsGameController;
pub use crate::stdlib::SDL_Joystick;
pub use crate::stdlib::SDL_JoystickClose;
pub use crate::stdlib::SDL_JoystickEventState;
pub use crate::stdlib::SDL_JoystickGetAxis;
pub use crate::stdlib::SDL_JoystickGetBall;
pub use crate::stdlib::SDL_JoystickGetButton;
pub use crate::stdlib::SDL_JoystickGetHat;
pub use crate::stdlib::SDL_JoystickID;
pub use crate::stdlib::SDL_JoystickNameForIndex;
pub use crate::stdlib::SDL_JoystickNumAxes;
pub use crate::stdlib::SDL_JoystickNumBalls;
pub use crate::stdlib::SDL_JoystickNumButtons;
pub use crate::stdlib::SDL_JoystickNumHats;
pub use crate::stdlib::SDL_JoystickOpen;
pub use crate::stdlib::SDL_JoystickUpdate;
pub use crate::stdlib::SDL_Keycode;
pub use crate::stdlib::SDL_Keysym;
pub use crate::stdlib::SDL_NumJoysticks;
pub use crate::stdlib::SDL_Scancode;
pub use crate::stdlib::SDL_SetWindowGrab;
pub use crate::stdlib::SDL_StartTextInput;
pub use crate::stdlib::SDL_StopTextInput;
pub use crate::stdlib::SDL_TouchID;
pub use crate::stdlib::SDL_Window;
pub use crate::stdlib::_SDL_GameController;
pub use crate::stdlib::_SDL_Joystick;
pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__int64_t;
pub use crate::stdlib::__uint16_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::__uint8_t;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::int64_t;
pub use crate::stdlib::uint16_t;
pub use crate::stdlib::uint32_t;
pub use crate::stdlib::uint8_t;
pub use crate::stdlib::SDL_bool;
pub use crate::stdlib::Sint16;
pub use crate::stdlib::Sint32;
pub use crate::stdlib::Sint64;
pub use crate::stdlib::Uint16;
pub use crate::stdlib::Uint32;
pub use crate::stdlib::Uint8;
pub use crate::stdlib::KMOD_CAPS;
pub use crate::stdlib::KMOD_LALT;
pub use crate::stdlib::KMOD_LCTRL;
pub use crate::stdlib::KMOD_LGUI;
pub use crate::stdlib::KMOD_LSHIFT;
pub use crate::stdlib::KMOD_MODE;
pub use crate::stdlib::KMOD_NONE;
pub use crate::stdlib::KMOD_NUM;
pub use crate::stdlib::KMOD_RALT;
pub use crate::stdlib::KMOD_RCTRL;
pub use crate::stdlib::KMOD_RESERVED;
pub use crate::stdlib::KMOD_RGUI;
pub use crate::stdlib::KMOD_RSHIFT;
pub use crate::stdlib::SDLK_0;
pub use crate::stdlib::SDLK_1;
pub use crate::stdlib::SDLK_2;
pub use crate::stdlib::SDLK_3;
pub use crate::stdlib::SDLK_4;
pub use crate::stdlib::SDLK_5;
pub use crate::stdlib::SDLK_6;
pub use crate::stdlib::SDLK_7;
pub use crate::stdlib::SDLK_8;
pub use crate::stdlib::SDLK_9;
pub use crate::stdlib::SDLK_AC_BACK;
pub use crate::stdlib::SDLK_AC_BOOKMARKS;
pub use crate::stdlib::SDLK_AC_FORWARD;
pub use crate::stdlib::SDLK_AC_HOME;
pub use crate::stdlib::SDLK_AC_REFRESH;
pub use crate::stdlib::SDLK_AC_SEARCH;
pub use crate::stdlib::SDLK_AC_STOP;
pub use crate::stdlib::SDLK_AGAIN;
pub use crate::stdlib::SDLK_ALTERASE;
pub use crate::stdlib::SDLK_AMPERSAND;
pub use crate::stdlib::SDLK_APP1;
pub use crate::stdlib::SDLK_APP2;
pub use crate::stdlib::SDLK_APPLICATION;
pub use crate::stdlib::SDLK_ASTERISK;
pub use crate::stdlib::SDLK_AT;
pub use crate::stdlib::SDLK_AUDIOFASTFORWARD;
pub use crate::stdlib::SDLK_AUDIOMUTE;
pub use crate::stdlib::SDLK_AUDIONEXT;
pub use crate::stdlib::SDLK_AUDIOPLAY;
pub use crate::stdlib::SDLK_AUDIOPREV;
pub use crate::stdlib::SDLK_AUDIOREWIND;
pub use crate::stdlib::SDLK_AUDIOSTOP;
pub use crate::stdlib::SDLK_BACKQUOTE;
pub use crate::stdlib::SDLK_BACKSLASH;
pub use crate::stdlib::SDLK_BACKSPACE;
pub use crate::stdlib::SDLK_BRIGHTNESSDOWN;
pub use crate::stdlib::SDLK_BRIGHTNESSUP;
pub use crate::stdlib::SDLK_CALCULATOR;
pub use crate::stdlib::SDLK_CANCEL;
pub use crate::stdlib::SDLK_CAPSLOCK;
pub use crate::stdlib::SDLK_CARET;
pub use crate::stdlib::SDLK_CLEAR;
pub use crate::stdlib::SDLK_CLEARAGAIN;
pub use crate::stdlib::SDLK_COLON;
pub use crate::stdlib::SDLK_COMMA;
pub use crate::stdlib::SDLK_COMPUTER;
pub use crate::stdlib::SDLK_COPY;
pub use crate::stdlib::SDLK_CRSEL;
pub use crate::stdlib::SDLK_CURRENCYSUBUNIT;
pub use crate::stdlib::SDLK_CURRENCYUNIT;
pub use crate::stdlib::SDLK_CUT;
pub use crate::stdlib::SDLK_DECIMALSEPARATOR;
pub use crate::stdlib::SDLK_DELETE;
pub use crate::stdlib::SDLK_DISPLAYSWITCH;
pub use crate::stdlib::SDLK_DOLLAR;
pub use crate::stdlib::SDLK_DOWN;
pub use crate::stdlib::SDLK_EJECT;
pub use crate::stdlib::SDLK_END;
pub use crate::stdlib::SDLK_EQUALS;
pub use crate::stdlib::SDLK_ESCAPE;
pub use crate::stdlib::SDLK_EXCLAIM;
pub use crate::stdlib::SDLK_EXECUTE;
pub use crate::stdlib::SDLK_EXSEL;
pub use crate::stdlib::SDLK_F1;
pub use crate::stdlib::SDLK_F10;
pub use crate::stdlib::SDLK_F11;
pub use crate::stdlib::SDLK_F12;
pub use crate::stdlib::SDLK_F13;
pub use crate::stdlib::SDLK_F14;
pub use crate::stdlib::SDLK_F15;
pub use crate::stdlib::SDLK_F16;
pub use crate::stdlib::SDLK_F17;
pub use crate::stdlib::SDLK_F18;
pub use crate::stdlib::SDLK_F19;
pub use crate::stdlib::SDLK_F2;
pub use crate::stdlib::SDLK_F20;
pub use crate::stdlib::SDLK_F21;
pub use crate::stdlib::SDLK_F22;
pub use crate::stdlib::SDLK_F23;
pub use crate::stdlib::SDLK_F24;
pub use crate::stdlib::SDLK_F3;
pub use crate::stdlib::SDLK_F4;
pub use crate::stdlib::SDLK_F5;
pub use crate::stdlib::SDLK_F6;
pub use crate::stdlib::SDLK_F7;
pub use crate::stdlib::SDLK_F8;
pub use crate::stdlib::SDLK_F9;
pub use crate::stdlib::SDLK_FIND;
pub use crate::stdlib::SDLK_GREATER;
pub use crate::stdlib::SDLK_HASH;
pub use crate::stdlib::SDLK_HELP;
pub use crate::stdlib::SDLK_HOME;
pub use crate::stdlib::SDLK_INSERT;
pub use crate::stdlib::SDLK_KBDILLUMDOWN;
pub use crate::stdlib::SDLK_KBDILLUMTOGGLE;
pub use crate::stdlib::SDLK_KBDILLUMUP;
pub use crate::stdlib::SDLK_KP_0;
pub use crate::stdlib::SDLK_KP_00;
pub use crate::stdlib::SDLK_KP_000;
pub use crate::stdlib::SDLK_KP_1;
pub use crate::stdlib::SDLK_KP_2;
pub use crate::stdlib::SDLK_KP_3;
pub use crate::stdlib::SDLK_KP_4;
pub use crate::stdlib::SDLK_KP_5;
pub use crate::stdlib::SDLK_KP_6;
pub use crate::stdlib::SDLK_KP_7;
pub use crate::stdlib::SDLK_KP_8;
pub use crate::stdlib::SDLK_KP_9;
pub use crate::stdlib::SDLK_KP_A;
pub use crate::stdlib::SDLK_KP_AMPERSAND;
pub use crate::stdlib::SDLK_KP_AT;
pub use crate::stdlib::SDLK_KP_B;
pub use crate::stdlib::SDLK_KP_BACKSPACE;
pub use crate::stdlib::SDLK_KP_BINARY;
pub use crate::stdlib::SDLK_KP_C;
pub use crate::stdlib::SDLK_KP_CLEAR;
pub use crate::stdlib::SDLK_KP_CLEARENTRY;
pub use crate::stdlib::SDLK_KP_COLON;
pub use crate::stdlib::SDLK_KP_COMMA;
pub use crate::stdlib::SDLK_KP_D;
pub use crate::stdlib::SDLK_KP_DBLAMPERSAND;
pub use crate::stdlib::SDLK_KP_DBLVERTICALBAR;
pub use crate::stdlib::SDLK_KP_DECIMAL;
pub use crate::stdlib::SDLK_KP_DIVIDE;
pub use crate::stdlib::SDLK_KP_E;
pub use crate::stdlib::SDLK_KP_ENTER;
pub use crate::stdlib::SDLK_KP_EQUALS;
pub use crate::stdlib::SDLK_KP_EQUALSAS400;
pub use crate::stdlib::SDLK_KP_EXCLAM;
pub use crate::stdlib::SDLK_KP_F;
pub use crate::stdlib::SDLK_KP_GREATER;
pub use crate::stdlib::SDLK_KP_HASH;
pub use crate::stdlib::SDLK_KP_HEXADECIMAL;
pub use crate::stdlib::SDLK_KP_LEFTBRACE;
pub use crate::stdlib::SDLK_KP_LEFTPAREN;
pub use crate::stdlib::SDLK_KP_LESS;
pub use crate::stdlib::SDLK_KP_MEMADD;
pub use crate::stdlib::SDLK_KP_MEMCLEAR;
pub use crate::stdlib::SDLK_KP_MEMDIVIDE;
pub use crate::stdlib::SDLK_KP_MEMMULTIPLY;
pub use crate::stdlib::SDLK_KP_MEMRECALL;
pub use crate::stdlib::SDLK_KP_MEMSTORE;
pub use crate::stdlib::SDLK_KP_MEMSUBTRACT;
pub use crate::stdlib::SDLK_KP_MINUS;
pub use crate::stdlib::SDLK_KP_MULTIPLY;
pub use crate::stdlib::SDLK_KP_OCTAL;
pub use crate::stdlib::SDLK_KP_PERCENT;
pub use crate::stdlib::SDLK_KP_PERIOD;
pub use crate::stdlib::SDLK_KP_PLUS;
pub use crate::stdlib::SDLK_KP_PLUSMINUS;
pub use crate::stdlib::SDLK_KP_POWER;
pub use crate::stdlib::SDLK_KP_RIGHTBRACE;
pub use crate::stdlib::SDLK_KP_RIGHTPAREN;
pub use crate::stdlib::SDLK_KP_SPACE;
pub use crate::stdlib::SDLK_KP_TAB;
pub use crate::stdlib::SDLK_KP_VERTICALBAR;
pub use crate::stdlib::SDLK_KP_XOR;
pub use crate::stdlib::SDLK_LALT;
pub use crate::stdlib::SDLK_LCTRL;
pub use crate::stdlib::SDLK_LEFT;
pub use crate::stdlib::SDLK_LEFTBRACKET;
pub use crate::stdlib::SDLK_LEFTPAREN;
pub use crate::stdlib::SDLK_LESS;
pub use crate::stdlib::SDLK_LGUI;
pub use crate::stdlib::SDLK_LSHIFT;
pub use crate::stdlib::SDLK_MAIL;
pub use crate::stdlib::SDLK_MEDIASELECT;
pub use crate::stdlib::SDLK_MENU;
pub use crate::stdlib::SDLK_MINUS;
pub use crate::stdlib::SDLK_MODE;
pub use crate::stdlib::SDLK_MUTE;
pub use crate::stdlib::SDLK_NUMLOCKCLEAR;
pub use crate::stdlib::SDLK_OPER;
pub use crate::stdlib::SDLK_OUT;
pub use crate::stdlib::SDLK_PAGEDOWN;
pub use crate::stdlib::SDLK_PAGEUP;
pub use crate::stdlib::SDLK_PASTE;
pub use crate::stdlib::SDLK_PAUSE;
pub use crate::stdlib::SDLK_PERCENT;
pub use crate::stdlib::SDLK_PERIOD;
pub use crate::stdlib::SDLK_PLUS;
pub use crate::stdlib::SDLK_POWER;
pub use crate::stdlib::SDLK_PRINTSCREEN;
pub use crate::stdlib::SDLK_PRIOR;
pub use crate::stdlib::SDLK_QUESTION;
pub use crate::stdlib::SDLK_QUOTE;
pub use crate::stdlib::SDLK_QUOTEDBL;
pub use crate::stdlib::SDLK_RALT;
pub use crate::stdlib::SDLK_RCTRL;
pub use crate::stdlib::SDLK_RETURN;
pub use crate::stdlib::SDLK_RETURN2;
pub use crate::stdlib::SDLK_RGUI;
pub use crate::stdlib::SDLK_RIGHT;
pub use crate::stdlib::SDLK_RIGHTBRACKET;
pub use crate::stdlib::SDLK_RIGHTPAREN;
pub use crate::stdlib::SDLK_RSHIFT;
pub use crate::stdlib::SDLK_SCROLLLOCK;
pub use crate::stdlib::SDLK_SELECT;
pub use crate::stdlib::SDLK_SEMICOLON;
pub use crate::stdlib::SDLK_SEPARATOR;
pub use crate::stdlib::SDLK_SLASH;
pub use crate::stdlib::SDLK_SLEEP;
pub use crate::stdlib::SDLK_SPACE;
pub use crate::stdlib::SDLK_STOP;
pub use crate::stdlib::SDLK_SYSREQ;
pub use crate::stdlib::SDLK_TAB;
pub use crate::stdlib::SDLK_THOUSANDSSEPARATOR;
pub use crate::stdlib::SDLK_UNDERSCORE;
pub use crate::stdlib::SDLK_UNDO;
pub use crate::stdlib::SDLK_UNKNOWN;
pub use crate::stdlib::SDLK_UP;
pub use crate::stdlib::SDLK_VOLUMEDOWN;
pub use crate::stdlib::SDLK_VOLUMEUP;
pub use crate::stdlib::SDLK_WWW;
pub use crate::stdlib::SDL_CONTROLLER_AXIS_INVALID;
pub use crate::stdlib::SDL_CONTROLLER_AXIS_LEFTX;
pub use crate::stdlib::SDL_CONTROLLER_AXIS_LEFTY;
pub use crate::stdlib::SDL_CONTROLLER_AXIS_MAX;
pub use crate::stdlib::SDL_CONTROLLER_AXIS_RIGHTX;
pub use crate::stdlib::SDL_CONTROLLER_AXIS_RIGHTY;
pub use crate::stdlib::SDL_CONTROLLER_AXIS_TRIGGERLEFT;
pub use crate::stdlib::SDL_CONTROLLER_AXIS_TRIGGERRIGHT;
pub use crate::stdlib::SDL_CONTROLLER_BUTTON_A;
pub use crate::stdlib::SDL_CONTROLLER_BUTTON_B;
pub use crate::stdlib::SDL_CONTROLLER_BUTTON_BACK;
pub use crate::stdlib::SDL_CONTROLLER_BUTTON_DPAD_DOWN;
pub use crate::stdlib::SDL_CONTROLLER_BUTTON_DPAD_LEFT;
pub use crate::stdlib::SDL_CONTROLLER_BUTTON_DPAD_RIGHT;
pub use crate::stdlib::SDL_CONTROLLER_BUTTON_DPAD_UP;
pub use crate::stdlib::SDL_CONTROLLER_BUTTON_GUIDE;
pub use crate::stdlib::SDL_CONTROLLER_BUTTON_INVALID;
pub use crate::stdlib::SDL_CONTROLLER_BUTTON_LEFTSHOULDER;
pub use crate::stdlib::SDL_CONTROLLER_BUTTON_LEFTSTICK;
pub use crate::stdlib::SDL_CONTROLLER_BUTTON_MAX;
pub use crate::stdlib::SDL_CONTROLLER_BUTTON_RIGHTSHOULDER;
pub use crate::stdlib::SDL_CONTROLLER_BUTTON_RIGHTSTICK;
pub use crate::stdlib::SDL_CONTROLLER_BUTTON_START;
pub use crate::stdlib::SDL_CONTROLLER_BUTTON_X;
pub use crate::stdlib::SDL_CONTROLLER_BUTTON_Y;
pub use crate::stdlib::SDL_FALSE;
pub use crate::stdlib::SDL_NUM_SCANCODES;
pub use crate::stdlib::SDL_SCANCODE_0;
pub use crate::stdlib::SDL_SCANCODE_1;
pub use crate::stdlib::SDL_SCANCODE_2;
pub use crate::stdlib::SDL_SCANCODE_3;
pub use crate::stdlib::SDL_SCANCODE_4;
pub use crate::stdlib::SDL_SCANCODE_5;
pub use crate::stdlib::SDL_SCANCODE_6;
pub use crate::stdlib::SDL_SCANCODE_7;
pub use crate::stdlib::SDL_SCANCODE_8;
pub use crate::stdlib::SDL_SCANCODE_9;
pub use crate::stdlib::SDL_SCANCODE_A;
pub use crate::stdlib::SDL_SCANCODE_AC_BACK;
pub use crate::stdlib::SDL_SCANCODE_AC_BOOKMARKS;
pub use crate::stdlib::SDL_SCANCODE_AC_FORWARD;
pub use crate::stdlib::SDL_SCANCODE_AC_HOME;
pub use crate::stdlib::SDL_SCANCODE_AC_REFRESH;
pub use crate::stdlib::SDL_SCANCODE_AC_SEARCH;
pub use crate::stdlib::SDL_SCANCODE_AC_STOP;
pub use crate::stdlib::SDL_SCANCODE_AGAIN;
pub use crate::stdlib::SDL_SCANCODE_ALTERASE;
pub use crate::stdlib::SDL_SCANCODE_APOSTROPHE;
pub use crate::stdlib::SDL_SCANCODE_APP1;
pub use crate::stdlib::SDL_SCANCODE_APP2;
pub use crate::stdlib::SDL_SCANCODE_APPLICATION;
pub use crate::stdlib::SDL_SCANCODE_AUDIOFASTFORWARD;
pub use crate::stdlib::SDL_SCANCODE_AUDIOMUTE;
pub use crate::stdlib::SDL_SCANCODE_AUDIONEXT;
pub use crate::stdlib::SDL_SCANCODE_AUDIOPLAY;
pub use crate::stdlib::SDL_SCANCODE_AUDIOPREV;
pub use crate::stdlib::SDL_SCANCODE_AUDIOREWIND;
pub use crate::stdlib::SDL_SCANCODE_AUDIOSTOP;
pub use crate::stdlib::SDL_SCANCODE_B;
pub use crate::stdlib::SDL_SCANCODE_BACKSLASH;
pub use crate::stdlib::SDL_SCANCODE_BACKSPACE;
pub use crate::stdlib::SDL_SCANCODE_BRIGHTNESSDOWN;
pub use crate::stdlib::SDL_SCANCODE_BRIGHTNESSUP;
pub use crate::stdlib::SDL_SCANCODE_C;
pub use crate::stdlib::SDL_SCANCODE_CALCULATOR;
pub use crate::stdlib::SDL_SCANCODE_CANCEL;
pub use crate::stdlib::SDL_SCANCODE_CAPSLOCK;
pub use crate::stdlib::SDL_SCANCODE_CLEAR;
pub use crate::stdlib::SDL_SCANCODE_CLEARAGAIN;
pub use crate::stdlib::SDL_SCANCODE_COMMA;
pub use crate::stdlib::SDL_SCANCODE_COMPUTER;
pub use crate::stdlib::SDL_SCANCODE_COPY;
pub use crate::stdlib::SDL_SCANCODE_CRSEL;
pub use crate::stdlib::SDL_SCANCODE_CURRENCYSUBUNIT;
pub use crate::stdlib::SDL_SCANCODE_CURRENCYUNIT;
pub use crate::stdlib::SDL_SCANCODE_CUT;
pub use crate::stdlib::SDL_SCANCODE_D;
pub use crate::stdlib::SDL_SCANCODE_DECIMALSEPARATOR;
pub use crate::stdlib::SDL_SCANCODE_DELETE;
pub use crate::stdlib::SDL_SCANCODE_DISPLAYSWITCH;
pub use crate::stdlib::SDL_SCANCODE_DOWN;
pub use crate::stdlib::SDL_SCANCODE_E;
pub use crate::stdlib::SDL_SCANCODE_EJECT;
pub use crate::stdlib::SDL_SCANCODE_END;
pub use crate::stdlib::SDL_SCANCODE_EQUALS;
pub use crate::stdlib::SDL_SCANCODE_ESCAPE;
pub use crate::stdlib::SDL_SCANCODE_EXECUTE;
pub use crate::stdlib::SDL_SCANCODE_EXSEL;
pub use crate::stdlib::SDL_SCANCODE_F;
pub use crate::stdlib::SDL_SCANCODE_F1;
pub use crate::stdlib::SDL_SCANCODE_F10;
pub use crate::stdlib::SDL_SCANCODE_F11;
pub use crate::stdlib::SDL_SCANCODE_F12;
pub use crate::stdlib::SDL_SCANCODE_F13;
pub use crate::stdlib::SDL_SCANCODE_F14;
pub use crate::stdlib::SDL_SCANCODE_F15;
pub use crate::stdlib::SDL_SCANCODE_F16;
pub use crate::stdlib::SDL_SCANCODE_F17;
pub use crate::stdlib::SDL_SCANCODE_F18;
pub use crate::stdlib::SDL_SCANCODE_F19;
pub use crate::stdlib::SDL_SCANCODE_F2;
pub use crate::stdlib::SDL_SCANCODE_F20;
pub use crate::stdlib::SDL_SCANCODE_F21;
pub use crate::stdlib::SDL_SCANCODE_F22;
pub use crate::stdlib::SDL_SCANCODE_F23;
pub use crate::stdlib::SDL_SCANCODE_F24;
pub use crate::stdlib::SDL_SCANCODE_F3;
pub use crate::stdlib::SDL_SCANCODE_F4;
pub use crate::stdlib::SDL_SCANCODE_F5;
pub use crate::stdlib::SDL_SCANCODE_F6;
pub use crate::stdlib::SDL_SCANCODE_F7;
pub use crate::stdlib::SDL_SCANCODE_F8;
pub use crate::stdlib::SDL_SCANCODE_F9;
pub use crate::stdlib::SDL_SCANCODE_FIND;
pub use crate::stdlib::SDL_SCANCODE_G;
pub use crate::stdlib::SDL_SCANCODE_GRAVE;
pub use crate::stdlib::SDL_SCANCODE_H;
pub use crate::stdlib::SDL_SCANCODE_HELP;
pub use crate::stdlib::SDL_SCANCODE_HOME;
pub use crate::stdlib::SDL_SCANCODE_I;
pub use crate::stdlib::SDL_SCANCODE_INSERT;
pub use crate::stdlib::SDL_SCANCODE_INTERNATIONAL1;
pub use crate::stdlib::SDL_SCANCODE_INTERNATIONAL2;
pub use crate::stdlib::SDL_SCANCODE_INTERNATIONAL3;
pub use crate::stdlib::SDL_SCANCODE_INTERNATIONAL4;
pub use crate::stdlib::SDL_SCANCODE_INTERNATIONAL5;
pub use crate::stdlib::SDL_SCANCODE_INTERNATIONAL6;
pub use crate::stdlib::SDL_SCANCODE_INTERNATIONAL7;
pub use crate::stdlib::SDL_SCANCODE_INTERNATIONAL8;
pub use crate::stdlib::SDL_SCANCODE_INTERNATIONAL9;
pub use crate::stdlib::SDL_SCANCODE_J;
pub use crate::stdlib::SDL_SCANCODE_K;
pub use crate::stdlib::SDL_SCANCODE_KBDILLUMDOWN;
pub use crate::stdlib::SDL_SCANCODE_KBDILLUMTOGGLE;
pub use crate::stdlib::SDL_SCANCODE_KBDILLUMUP;
pub use crate::stdlib::SDL_SCANCODE_KP_0;
pub use crate::stdlib::SDL_SCANCODE_KP_00;
pub use crate::stdlib::SDL_SCANCODE_KP_000;
pub use crate::stdlib::SDL_SCANCODE_KP_1;
pub use crate::stdlib::SDL_SCANCODE_KP_2;
pub use crate::stdlib::SDL_SCANCODE_KP_3;
pub use crate::stdlib::SDL_SCANCODE_KP_4;
pub use crate::stdlib::SDL_SCANCODE_KP_5;
pub use crate::stdlib::SDL_SCANCODE_KP_6;
pub use crate::stdlib::SDL_SCANCODE_KP_7;
pub use crate::stdlib::SDL_SCANCODE_KP_8;
pub use crate::stdlib::SDL_SCANCODE_KP_9;
pub use crate::stdlib::SDL_SCANCODE_KP_A;
pub use crate::stdlib::SDL_SCANCODE_KP_AMPERSAND;
pub use crate::stdlib::SDL_SCANCODE_KP_AT;
pub use crate::stdlib::SDL_SCANCODE_KP_B;
pub use crate::stdlib::SDL_SCANCODE_KP_BACKSPACE;
pub use crate::stdlib::SDL_SCANCODE_KP_BINARY;
pub use crate::stdlib::SDL_SCANCODE_KP_C;
pub use crate::stdlib::SDL_SCANCODE_KP_CLEAR;
pub use crate::stdlib::SDL_SCANCODE_KP_CLEARENTRY;
pub use crate::stdlib::SDL_SCANCODE_KP_COLON;
pub use crate::stdlib::SDL_SCANCODE_KP_COMMA;
pub use crate::stdlib::SDL_SCANCODE_KP_D;
pub use crate::stdlib::SDL_SCANCODE_KP_DBLAMPERSAND;
pub use crate::stdlib::SDL_SCANCODE_KP_DBLVERTICALBAR;
pub use crate::stdlib::SDL_SCANCODE_KP_DECIMAL;
pub use crate::stdlib::SDL_SCANCODE_KP_DIVIDE;
pub use crate::stdlib::SDL_SCANCODE_KP_E;
pub use crate::stdlib::SDL_SCANCODE_KP_ENTER;
pub use crate::stdlib::SDL_SCANCODE_KP_EQUALS;
pub use crate::stdlib::SDL_SCANCODE_KP_EQUALSAS400;
pub use crate::stdlib::SDL_SCANCODE_KP_EXCLAM;
pub use crate::stdlib::SDL_SCANCODE_KP_F;
pub use crate::stdlib::SDL_SCANCODE_KP_GREATER;
pub use crate::stdlib::SDL_SCANCODE_KP_HASH;
pub use crate::stdlib::SDL_SCANCODE_KP_HEXADECIMAL;
pub use crate::stdlib::SDL_SCANCODE_KP_LEFTBRACE;
pub use crate::stdlib::SDL_SCANCODE_KP_LEFTPAREN;
pub use crate::stdlib::SDL_SCANCODE_KP_LESS;
pub use crate::stdlib::SDL_SCANCODE_KP_MEMADD;
pub use crate::stdlib::SDL_SCANCODE_KP_MEMCLEAR;
pub use crate::stdlib::SDL_SCANCODE_KP_MEMDIVIDE;
pub use crate::stdlib::SDL_SCANCODE_KP_MEMMULTIPLY;
pub use crate::stdlib::SDL_SCANCODE_KP_MEMRECALL;
pub use crate::stdlib::SDL_SCANCODE_KP_MEMSTORE;
pub use crate::stdlib::SDL_SCANCODE_KP_MEMSUBTRACT;
pub use crate::stdlib::SDL_SCANCODE_KP_MINUS;
pub use crate::stdlib::SDL_SCANCODE_KP_MULTIPLY;
pub use crate::stdlib::SDL_SCANCODE_KP_OCTAL;
pub use crate::stdlib::SDL_SCANCODE_KP_PERCENT;
pub use crate::stdlib::SDL_SCANCODE_KP_PERIOD;
pub use crate::stdlib::SDL_SCANCODE_KP_PLUS;
pub use crate::stdlib::SDL_SCANCODE_KP_PLUSMINUS;
pub use crate::stdlib::SDL_SCANCODE_KP_POWER;
pub use crate::stdlib::SDL_SCANCODE_KP_RIGHTBRACE;
pub use crate::stdlib::SDL_SCANCODE_KP_RIGHTPAREN;
pub use crate::stdlib::SDL_SCANCODE_KP_SPACE;
pub use crate::stdlib::SDL_SCANCODE_KP_TAB;
pub use crate::stdlib::SDL_SCANCODE_KP_VERTICALBAR;
pub use crate::stdlib::SDL_SCANCODE_KP_XOR;
pub use crate::stdlib::SDL_SCANCODE_L;
pub use crate::stdlib::SDL_SCANCODE_LALT;
pub use crate::stdlib::SDL_SCANCODE_LANG1;
pub use crate::stdlib::SDL_SCANCODE_LANG2;
pub use crate::stdlib::SDL_SCANCODE_LANG3;
pub use crate::stdlib::SDL_SCANCODE_LANG4;
pub use crate::stdlib::SDL_SCANCODE_LANG5;
pub use crate::stdlib::SDL_SCANCODE_LANG6;
pub use crate::stdlib::SDL_SCANCODE_LANG7;
pub use crate::stdlib::SDL_SCANCODE_LANG8;
pub use crate::stdlib::SDL_SCANCODE_LANG9;
pub use crate::stdlib::SDL_SCANCODE_LCTRL;
pub use crate::stdlib::SDL_SCANCODE_LEFT;
pub use crate::stdlib::SDL_SCANCODE_LEFTBRACKET;
pub use crate::stdlib::SDL_SCANCODE_LGUI;
pub use crate::stdlib::SDL_SCANCODE_LSHIFT;
pub use crate::stdlib::SDL_SCANCODE_M;
pub use crate::stdlib::SDL_SCANCODE_MAIL;
pub use crate::stdlib::SDL_SCANCODE_MEDIASELECT;
pub use crate::stdlib::SDL_SCANCODE_MENU;
pub use crate::stdlib::SDL_SCANCODE_MINUS;
pub use crate::stdlib::SDL_SCANCODE_MODE;
pub use crate::stdlib::SDL_SCANCODE_MUTE;
pub use crate::stdlib::SDL_SCANCODE_N;
pub use crate::stdlib::SDL_SCANCODE_NONUSBACKSLASH;
pub use crate::stdlib::SDL_SCANCODE_NONUSHASH;
pub use crate::stdlib::SDL_SCANCODE_NUMLOCKCLEAR;
pub use crate::stdlib::SDL_SCANCODE_O;
pub use crate::stdlib::SDL_SCANCODE_OPER;
pub use crate::stdlib::SDL_SCANCODE_OUT;
pub use crate::stdlib::SDL_SCANCODE_P;
pub use crate::stdlib::SDL_SCANCODE_PAGEDOWN;
pub use crate::stdlib::SDL_SCANCODE_PAGEUP;
pub use crate::stdlib::SDL_SCANCODE_PASTE;
pub use crate::stdlib::SDL_SCANCODE_PAUSE;
pub use crate::stdlib::SDL_SCANCODE_PERIOD;
pub use crate::stdlib::SDL_SCANCODE_POWER;
pub use crate::stdlib::SDL_SCANCODE_PRINTSCREEN;
pub use crate::stdlib::SDL_SCANCODE_PRIOR;
pub use crate::stdlib::SDL_SCANCODE_Q;
pub use crate::stdlib::SDL_SCANCODE_R;
pub use crate::stdlib::SDL_SCANCODE_RALT;
pub use crate::stdlib::SDL_SCANCODE_RCTRL;
pub use crate::stdlib::SDL_SCANCODE_RETURN;
pub use crate::stdlib::SDL_SCANCODE_RETURN2;
pub use crate::stdlib::SDL_SCANCODE_RGUI;
pub use crate::stdlib::SDL_SCANCODE_RIGHT;
pub use crate::stdlib::SDL_SCANCODE_RIGHTBRACKET;
pub use crate::stdlib::SDL_SCANCODE_RSHIFT;
pub use crate::stdlib::SDL_SCANCODE_S;
pub use crate::stdlib::SDL_SCANCODE_SCROLLLOCK;
pub use crate::stdlib::SDL_SCANCODE_SELECT;
pub use crate::stdlib::SDL_SCANCODE_SEMICOLON;
pub use crate::stdlib::SDL_SCANCODE_SEPARATOR;
pub use crate::stdlib::SDL_SCANCODE_SLASH;
pub use crate::stdlib::SDL_SCANCODE_SLEEP;
pub use crate::stdlib::SDL_SCANCODE_SPACE;
pub use crate::stdlib::SDL_SCANCODE_STOP;
pub use crate::stdlib::SDL_SCANCODE_SYSREQ;
pub use crate::stdlib::SDL_SCANCODE_T;
pub use crate::stdlib::SDL_SCANCODE_TAB;
pub use crate::stdlib::SDL_SCANCODE_THOUSANDSSEPARATOR;
pub use crate::stdlib::SDL_SCANCODE_U;
pub use crate::stdlib::SDL_SCANCODE_UNDO;
pub use crate::stdlib::SDL_SCANCODE_UNKNOWN;
pub use crate::stdlib::SDL_SCANCODE_UP;
pub use crate::stdlib::SDL_SCANCODE_V;
pub use crate::stdlib::SDL_SCANCODE_VOLUMEDOWN;
pub use crate::stdlib::SDL_SCANCODE_VOLUMEUP;
pub use crate::stdlib::SDL_SCANCODE_W;
pub use crate::stdlib::SDL_SCANCODE_WWW;
pub use crate::stdlib::SDL_SCANCODE_X;
pub use crate::stdlib::SDL_SCANCODE_Y;
pub use crate::stdlib::SDL_SCANCODE_Z;
pub use crate::stdlib::SDL_TRUE;
pub use crate::stdlib::SDL_WINDOWEVENT_CLOSE;
pub use crate::stdlib::SDL_WINDOWEVENT_ENTER;
pub use crate::stdlib::SDL_WINDOWEVENT_EXPOSED;
pub use crate::stdlib::SDL_WINDOWEVENT_FOCUS_GAINED;
pub use crate::stdlib::SDL_WINDOWEVENT_FOCUS_LOST;
pub use crate::stdlib::SDL_WINDOWEVENT_HIDDEN;
pub use crate::stdlib::SDL_WINDOWEVENT_HIT_TEST;
pub use crate::stdlib::SDL_WINDOWEVENT_LEAVE;
pub use crate::stdlib::SDL_WINDOWEVENT_MAXIMIZED;
pub use crate::stdlib::SDL_WINDOWEVENT_MINIMIZED;
pub use crate::stdlib::SDL_WINDOWEVENT_MOVED;
pub use crate::stdlib::SDL_WINDOWEVENT_NONE;
pub use crate::stdlib::SDL_WINDOWEVENT_RESIZED;
pub use crate::stdlib::SDL_WINDOWEVENT_RESTORED;
pub use crate::stdlib::SDL_WINDOWEVENT_SHOWN;
pub use crate::stdlib::SDL_WINDOWEVENT_SIZE_CHANGED;
pub use crate::stdlib::SDL_WINDOWEVENT_TAKE_FOCUS;
pub use crate::stdlib::SDL_WINDOW_ALLOW_HIGHDPI;
pub use crate::stdlib::SDL_WINDOW_ALWAYS_ON_TOP;
pub use crate::stdlib::SDL_WINDOW_BORDERLESS;
pub use crate::stdlib::SDL_WINDOW_FOREIGN;
pub use crate::stdlib::SDL_WINDOW_FULLSCREEN;
pub use crate::stdlib::SDL_WINDOW_FULLSCREEN_DESKTOP;
pub use crate::stdlib::SDL_WINDOW_HIDDEN;
pub use crate::stdlib::SDL_WINDOW_INPUT_FOCUS;
pub use crate::stdlib::SDL_WINDOW_INPUT_GRABBED;
pub use crate::stdlib::SDL_WINDOW_MAXIMIZED;
pub use crate::stdlib::SDL_WINDOW_MINIMIZED;
pub use crate::stdlib::SDL_WINDOW_MOUSE_CAPTURE;
pub use crate::stdlib::SDL_WINDOW_MOUSE_FOCUS;
pub use crate::stdlib::SDL_WINDOW_OPENGL;
pub use crate::stdlib::SDL_WINDOW_POPUP_MENU;
pub use crate::stdlib::SDL_WINDOW_RESIZABLE;
pub use crate::stdlib::SDL_WINDOW_SHOWN;
pub use crate::stdlib::SDL_WINDOW_SKIP_TASKBAR;
pub use crate::stdlib::SDL_WINDOW_TOOLTIP;
pub use crate::stdlib::SDL_WINDOW_UTILITY;
pub use crate::stdlib::SDL_WINDOW_VULKAN;

pub use crate::client_h::clientConnection_t;
pub use crate::client_h::clientStatic_t;
pub use crate::client_h::serverInfo_t;
pub use crate::curl_h::CURL;
pub use crate::keycodes_h::keyNum_t;
pub use crate::keycodes_h::K_ALT;
pub use crate::keycodes_h::K_AUX1;
pub use crate::keycodes_h::K_AUX10;
pub use crate::keycodes_h::K_AUX11;
pub use crate::keycodes_h::K_AUX12;
pub use crate::keycodes_h::K_AUX13;
pub use crate::keycodes_h::K_AUX14;
pub use crate::keycodes_h::K_AUX15;
pub use crate::keycodes_h::K_AUX16;
pub use crate::keycodes_h::K_AUX2;
pub use crate::keycodes_h::K_AUX3;
pub use crate::keycodes_h::K_AUX4;
pub use crate::keycodes_h::K_AUX5;
pub use crate::keycodes_h::K_AUX6;
pub use crate::keycodes_h::K_AUX7;
pub use crate::keycodes_h::K_AUX8;
pub use crate::keycodes_h::K_AUX9;
pub use crate::keycodes_h::K_BACKSPACE;
pub use crate::keycodes_h::K_BREAK;
pub use crate::keycodes_h::K_CAPSLOCK;
pub use crate::keycodes_h::K_COMMAND;
pub use crate::keycodes_h::K_COMPOSE;
pub use crate::keycodes_h::K_CONSOLE;
pub use crate::keycodes_h::K_CTRL;
pub use crate::keycodes_h::K_DEL;
pub use crate::keycodes_h::K_DOWNARROW;
pub use crate::keycodes_h::K_END;
pub use crate::keycodes_h::K_ENTER;
pub use crate::keycodes_h::K_ESCAPE;
pub use crate::keycodes_h::K_EURO;
pub use crate::keycodes_h::K_F1;
pub use crate::keycodes_h::K_F10;
pub use crate::keycodes_h::K_F11;
pub use crate::keycodes_h::K_F12;
pub use crate::keycodes_h::K_F13;
pub use crate::keycodes_h::K_F14;
pub use crate::keycodes_h::K_F15;
pub use crate::keycodes_h::K_F2;
pub use crate::keycodes_h::K_F3;
pub use crate::keycodes_h::K_F4;
pub use crate::keycodes_h::K_F5;
pub use crate::keycodes_h::K_F6;
pub use crate::keycodes_h::K_F7;
pub use crate::keycodes_h::K_F8;
pub use crate::keycodes_h::K_F9;
pub use crate::keycodes_h::K_HELP;
pub use crate::keycodes_h::K_HOME;
pub use crate::keycodes_h::K_INS;
pub use crate::keycodes_h::K_JOY1;
pub use crate::keycodes_h::K_JOY10;
pub use crate::keycodes_h::K_JOY11;
pub use crate::keycodes_h::K_JOY12;
pub use crate::keycodes_h::K_JOY13;
pub use crate::keycodes_h::K_JOY14;
pub use crate::keycodes_h::K_JOY15;
pub use crate::keycodes_h::K_JOY16;
pub use crate::keycodes_h::K_JOY17;
pub use crate::keycodes_h::K_JOY18;
pub use crate::keycodes_h::K_JOY19;
pub use crate::keycodes_h::K_JOY2;
pub use crate::keycodes_h::K_JOY20;
pub use crate::keycodes_h::K_JOY21;
pub use crate::keycodes_h::K_JOY22;
pub use crate::keycodes_h::K_JOY23;
pub use crate::keycodes_h::K_JOY24;
pub use crate::keycodes_h::K_JOY25;
pub use crate::keycodes_h::K_JOY26;
pub use crate::keycodes_h::K_JOY27;
pub use crate::keycodes_h::K_JOY28;
pub use crate::keycodes_h::K_JOY29;
pub use crate::keycodes_h::K_JOY3;
pub use crate::keycodes_h::K_JOY30;
pub use crate::keycodes_h::K_JOY31;
pub use crate::keycodes_h::K_JOY32;
pub use crate::keycodes_h::K_JOY4;
pub use crate::keycodes_h::K_JOY5;
pub use crate::keycodes_h::K_JOY6;
pub use crate::keycodes_h::K_JOY7;
pub use crate::keycodes_h::K_JOY8;
pub use crate::keycodes_h::K_JOY9;
pub use crate::keycodes_h::K_KP_5;
pub use crate::keycodes_h::K_KP_DEL;
pub use crate::keycodes_h::K_KP_DOWNARROW;
pub use crate::keycodes_h::K_KP_END;
pub use crate::keycodes_h::K_KP_ENTER;
pub use crate::keycodes_h::K_KP_EQUALS;
pub use crate::keycodes_h::K_KP_HOME;
pub use crate::keycodes_h::K_KP_INS;
pub use crate::keycodes_h::K_KP_LEFTARROW;
pub use crate::keycodes_h::K_KP_MINUS;
pub use crate::keycodes_h::K_KP_NUMLOCK;
pub use crate::keycodes_h::K_KP_PGDN;
pub use crate::keycodes_h::K_KP_PGUP;
pub use crate::keycodes_h::K_KP_PLUS;
pub use crate::keycodes_h::K_KP_RIGHTARROW;
pub use crate::keycodes_h::K_KP_SLASH;
pub use crate::keycodes_h::K_KP_STAR;
pub use crate::keycodes_h::K_KP_UPARROW;
pub use crate::keycodes_h::K_LEFTARROW;
pub use crate::keycodes_h::K_MENU;
pub use crate::keycodes_h::K_MODE;
pub use crate::keycodes_h::K_MOUSE1;
pub use crate::keycodes_h::K_MOUSE2;
pub use crate::keycodes_h::K_MOUSE3;
pub use crate::keycodes_h::K_MOUSE4;
pub use crate::keycodes_h::K_MOUSE5;
pub use crate::keycodes_h::K_MWHEELDOWN;
pub use crate::keycodes_h::K_MWHEELUP;
pub use crate::keycodes_h::K_PAD0_A;
pub use crate::keycodes_h::K_PAD0_B;
pub use crate::keycodes_h::K_PAD0_BACK;
pub use crate::keycodes_h::K_PAD0_DPAD_DOWN;
pub use crate::keycodes_h::K_PAD0_DPAD_LEFT;
pub use crate::keycodes_h::K_PAD0_DPAD_RIGHT;
pub use crate::keycodes_h::K_PAD0_DPAD_UP;
pub use crate::keycodes_h::K_PAD0_GUIDE;
pub use crate::keycodes_h::K_PAD0_LEFTSHOULDER;
pub use crate::keycodes_h::K_PAD0_LEFTSTICK_CLICK;
pub use crate::keycodes_h::K_PAD0_LEFTSTICK_DOWN;
pub use crate::keycodes_h::K_PAD0_LEFTSTICK_LEFT;
pub use crate::keycodes_h::K_PAD0_LEFTSTICK_RIGHT;
pub use crate::keycodes_h::K_PAD0_LEFTSTICK_UP;
pub use crate::keycodes_h::K_PAD0_LEFTTRIGGER;
pub use crate::keycodes_h::K_PAD0_RIGHTSHOULDER;
pub use crate::keycodes_h::K_PAD0_RIGHTSTICK_CLICK;
pub use crate::keycodes_h::K_PAD0_RIGHTSTICK_DOWN;
pub use crate::keycodes_h::K_PAD0_RIGHTSTICK_LEFT;
pub use crate::keycodes_h::K_PAD0_RIGHTSTICK_RIGHT;
pub use crate::keycodes_h::K_PAD0_RIGHTSTICK_UP;
pub use crate::keycodes_h::K_PAD0_RIGHTTRIGGER;
pub use crate::keycodes_h::K_PAD0_START;
pub use crate::keycodes_h::K_PAD0_X;
pub use crate::keycodes_h::K_PAD0_Y;
pub use crate::keycodes_h::K_PAUSE;
pub use crate::keycodes_h::K_PGDN;
pub use crate::keycodes_h::K_PGUP;
pub use crate::keycodes_h::K_POWER;
pub use crate::keycodes_h::K_PRINT;
pub use crate::keycodes_h::K_RIGHTARROW;
pub use crate::keycodes_h::K_SCROLLOCK;
pub use crate::keycodes_h::K_SHIFT;
pub use crate::keycodes_h::K_SPACE;
pub use crate::keycodes_h::K_SUPER;
pub use crate::keycodes_h::K_SYSREQ;
pub use crate::keycodes_h::K_TAB;
pub use crate::keycodes_h::K_UNDO;
pub use crate::keycodes_h::K_UPARROW;
pub use crate::keycodes_h::K_WORLD_0;
pub use crate::keycodes_h::K_WORLD_1;
pub use crate::keycodes_h::K_WORLD_10;
pub use crate::keycodes_h::K_WORLD_11;
pub use crate::keycodes_h::K_WORLD_12;
pub use crate::keycodes_h::K_WORLD_13;
pub use crate::keycodes_h::K_WORLD_14;
pub use crate::keycodes_h::K_WORLD_15;
pub use crate::keycodes_h::K_WORLD_16;
pub use crate::keycodes_h::K_WORLD_17;
pub use crate::keycodes_h::K_WORLD_18;
pub use crate::keycodes_h::K_WORLD_19;
pub use crate::keycodes_h::K_WORLD_2;
pub use crate::keycodes_h::K_WORLD_20;
pub use crate::keycodes_h::K_WORLD_21;
pub use crate::keycodes_h::K_WORLD_22;
pub use crate::keycodes_h::K_WORLD_23;
pub use crate::keycodes_h::K_WORLD_24;
pub use crate::keycodes_h::K_WORLD_25;
pub use crate::keycodes_h::K_WORLD_26;
pub use crate::keycodes_h::K_WORLD_27;
pub use crate::keycodes_h::K_WORLD_28;
pub use crate::keycodes_h::K_WORLD_29;
pub use crate::keycodes_h::K_WORLD_3;
pub use crate::keycodes_h::K_WORLD_30;
pub use crate::keycodes_h::K_WORLD_31;
pub use crate::keycodes_h::K_WORLD_32;
pub use crate::keycodes_h::K_WORLD_33;
pub use crate::keycodes_h::K_WORLD_34;
pub use crate::keycodes_h::K_WORLD_35;
pub use crate::keycodes_h::K_WORLD_36;
pub use crate::keycodes_h::K_WORLD_37;
pub use crate::keycodes_h::K_WORLD_38;
pub use crate::keycodes_h::K_WORLD_39;
pub use crate::keycodes_h::K_WORLD_4;
pub use crate::keycodes_h::K_WORLD_40;
pub use crate::keycodes_h::K_WORLD_41;
pub use crate::keycodes_h::K_WORLD_42;
pub use crate::keycodes_h::K_WORLD_43;
pub use crate::keycodes_h::K_WORLD_44;
pub use crate::keycodes_h::K_WORLD_45;
pub use crate::keycodes_h::K_WORLD_46;
pub use crate::keycodes_h::K_WORLD_47;
pub use crate::keycodes_h::K_WORLD_48;
pub use crate::keycodes_h::K_WORLD_49;
pub use crate::keycodes_h::K_WORLD_5;
pub use crate::keycodes_h::K_WORLD_50;
pub use crate::keycodes_h::K_WORLD_51;
pub use crate::keycodes_h::K_WORLD_52;
pub use crate::keycodes_h::K_WORLD_53;
pub use crate::keycodes_h::K_WORLD_54;
pub use crate::keycodes_h::K_WORLD_55;
pub use crate::keycodes_h::K_WORLD_56;
pub use crate::keycodes_h::K_WORLD_57;
pub use crate::keycodes_h::K_WORLD_58;
pub use crate::keycodes_h::K_WORLD_59;
pub use crate::keycodes_h::K_WORLD_6;
pub use crate::keycodes_h::K_WORLD_60;
pub use crate::keycodes_h::K_WORLD_61;
pub use crate::keycodes_h::K_WORLD_62;
pub use crate::keycodes_h::K_WORLD_63;
pub use crate::keycodes_h::K_WORLD_64;
pub use crate::keycodes_h::K_WORLD_65;
pub use crate::keycodes_h::K_WORLD_66;
pub use crate::keycodes_h::K_WORLD_67;
pub use crate::keycodes_h::K_WORLD_68;
pub use crate::keycodes_h::K_WORLD_69;
pub use crate::keycodes_h::K_WORLD_7;
pub use crate::keycodes_h::K_WORLD_70;
pub use crate::keycodes_h::K_WORLD_71;
pub use crate::keycodes_h::K_WORLD_72;
pub use crate::keycodes_h::K_WORLD_73;
pub use crate::keycodes_h::K_WORLD_74;
pub use crate::keycodes_h::K_WORLD_75;
pub use crate::keycodes_h::K_WORLD_76;
pub use crate::keycodes_h::K_WORLD_77;
pub use crate::keycodes_h::K_WORLD_78;
pub use crate::keycodes_h::K_WORLD_79;
pub use crate::keycodes_h::K_WORLD_8;
pub use crate::keycodes_h::K_WORLD_80;
pub use crate::keycodes_h::K_WORLD_81;
pub use crate::keycodes_h::K_WORLD_82;
pub use crate::keycodes_h::K_WORLD_83;
pub use crate::keycodes_h::K_WORLD_84;
pub use crate::keycodes_h::K_WORLD_85;
pub use crate::keycodes_h::K_WORLD_86;
pub use crate::keycodes_h::K_WORLD_87;
pub use crate::keycodes_h::K_WORLD_88;
pub use crate::keycodes_h::K_WORLD_89;
pub use crate::keycodes_h::K_WORLD_9;
pub use crate::keycodes_h::K_WORLD_90;
pub use crate::keycodes_h::K_WORLD_91;
pub use crate::keycodes_h::K_WORLD_92;
pub use crate::keycodes_h::K_WORLD_93;
pub use crate::keycodes_h::K_WORLD_94;
pub use crate::keycodes_h::K_WORLD_95;
pub use crate::keycodes_h::MAX_KEYS;
pub use crate::keys_h::qkey_t;
pub use crate::multi_h::CURLM;
pub use crate::qcommon_h::netadr_t;
pub use crate::qcommon_h::netadrtype_t;
pub use crate::qcommon_h::netchan_t;
pub use crate::qcommon_h::netsrc_t;
pub use crate::qcommon_h::sysEventType_t;
pub use crate::qcommon_h::NA_BAD;
pub use crate::qcommon_h::NA_BOT;
pub use crate::qcommon_h::NA_BROADCAST;
pub use crate::qcommon_h::NA_IP;
pub use crate::qcommon_h::NA_IP6;
pub use crate::qcommon_h::NA_LOOPBACK;
pub use crate::qcommon_h::NA_MULTICAST6;
pub use crate::qcommon_h::NA_UNSPEC;
pub use crate::qcommon_h::NS_CLIENT;
pub use crate::qcommon_h::NS_SERVER;
pub use crate::qcommon_h::SE_CHAR;
pub use crate::qcommon_h::SE_CONSOLE;
pub use crate::qcommon_h::SE_JOYSTICK_AXIS;
pub use crate::qcommon_h::SE_KEY;
pub use crate::qcommon_h::SE_MOUSE;
pub use crate::qcommon_h::SE_NONE;
pub use crate::src::client::cl_keys::keys;
pub use crate::src::client::cl_keys::Key_GetBinding;
pub use crate::src::client::cl_keys::Key_GetCatcher;
pub use crate::src::client::cl_keys::Key_KeynumToString;
pub use crate::src::client::cl_keys::Key_StringToKeynum;
pub use crate::src::client::cl_main::cl_consoleKeys;
pub use crate::src::client::cl_main::clc;
pub use crate::src::client::cl_main::cls;
pub use crate::src::client::cl_main::j_forward;
pub use crate::src::client::cl_main::j_forward_axis;
pub use crate::src::client::cl_main::j_pitch;
pub use crate::src::client::cl_main::j_pitch_axis;
pub use crate::src::client::cl_main::j_side;
pub use crate::src::client::cl_main::j_side_axis;
pub use crate::src::client::cl_main::j_up;
pub use crate::src::client::cl_main::j_up_axis;
pub use crate::src::client::cl_main::j_yaw;
pub use crate::src::client::cl_main::j_yaw_axis;

pub use crate::src::qcommon::cmd::Cbuf_AddText;
pub use crate::src::qcommon::cmd::Cbuf_ExecuteText;
pub use crate::src::qcommon::common::Com_DPrintf;
pub use crate::src::qcommon::common::Com_Error;
pub use crate::src::qcommon::common::Com_Printf;
pub use crate::src::qcommon::common::Com_QueueEvent;
pub use crate::src::qcommon::cvar::Cvar_Get;
pub use crate::src::qcommon::cvar::Cvar_Set;
pub use crate::src::qcommon::cvar::Cvar_SetValue;
pub use crate::src::qcommon::cvar::Cvar_VariableIntegerValue;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::connstate_t;
pub use crate::src::qcommon::q_shared::cvar_s;
pub use crate::src::qcommon::q_shared::cvar_t;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::COM_Parse;
pub use crate::src::qcommon::q_shared::Com_HexStrToInt;
pub use crate::src::qcommon::q_shared::Q_strcat;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::CA_ACTIVE;
pub use crate::src::qcommon::q_shared::CA_AUTHORIZING;
pub use crate::src::qcommon::q_shared::CA_CHALLENGING;
pub use crate::src::qcommon::q_shared::CA_CINEMATIC;
pub use crate::src::qcommon::q_shared::CA_CONNECTED;
pub use crate::src::qcommon::q_shared::CA_CONNECTING;
pub use crate::src::qcommon::q_shared::CA_DISCONNECTED;
pub use crate::src::qcommon::q_shared::CA_LOADING;
pub use crate::src::qcommon::q_shared::CA_PRIMED;
pub use crate::src::qcommon::q_shared::CA_UNINITIALIZED;
pub use crate::src::qcommon::q_shared::ERR_DISCONNECT;
pub use crate::src::qcommon::q_shared::ERR_DROP;
pub use crate::src::qcommon::q_shared::ERR_FATAL;
pub use crate::src::qcommon::q_shared::ERR_NEED_CD;
pub use crate::src::qcommon::q_shared::ERR_SERVERDISCONNECT;
pub use crate::src::qcommon::q_shared::EXEC_APPEND;
pub use crate::src::qcommon::q_shared::EXEC_INSERT;
pub use crate::src::qcommon::q_shared::EXEC_NOW;
pub use crate::src::sys::sys_unix::Sys_Milliseconds;

pub use crate::stdlib::SDL_AudioDeviceEvent;
pub use crate::stdlib::SDL_CommonEvent;
pub use crate::stdlib::SDL_ControllerAxisEvent;
pub use crate::stdlib::SDL_ControllerButtonEvent;
pub use crate::stdlib::SDL_ControllerDeviceEvent;
pub use crate::stdlib::SDL_DisplayEvent;
pub use crate::stdlib::SDL_DollarGestureEvent;
pub use crate::stdlib::SDL_DropEvent;
pub use crate::stdlib::SDL_Event;

pub use crate::stdlib::SDL_JoyAxisEvent;
pub use crate::stdlib::SDL_JoyBallEvent;
pub use crate::stdlib::SDL_JoyButtonEvent;
pub use crate::stdlib::SDL_JoyDeviceEvent;
pub use crate::stdlib::SDL_JoyHatEvent;
pub use crate::stdlib::SDL_KeyboardEvent;
pub use crate::stdlib::SDL_MouseButtonEvent;
pub use crate::stdlib::SDL_MouseMotionEvent;
pub use crate::stdlib::SDL_MouseWheelEvent;
pub use crate::stdlib::SDL_MultiGestureEvent;
pub use crate::stdlib::SDL_PeepEvents;
pub use crate::stdlib::SDL_PollEvent;
pub use crate::stdlib::SDL_PumpEvents;
pub use crate::stdlib::SDL_QuitEvent;
pub use crate::stdlib::SDL_SensorEvent;
pub use crate::stdlib::SDL_SysWMEvent;
pub use crate::stdlib::SDL_SysWMmsg;
pub use crate::stdlib::SDL_TextEditingEvent;
pub use crate::stdlib::SDL_TextInputEvent;
pub use crate::stdlib::SDL_TouchFingerEvent;
pub use crate::stdlib::SDL_UserEvent;
pub use crate::stdlib::SDL_WindowEvent;
pub use crate::stdlib::SDL_eventaction;
pub use crate::stdlib::SDL_ADDEVENT;
pub use crate::stdlib::SDL_APP_DIDENTERBACKGROUND;
pub use crate::stdlib::SDL_APP_DIDENTERFOREGROUND;
pub use crate::stdlib::SDL_APP_LOWMEMORY;
pub use crate::stdlib::SDL_APP_TERMINATING;
pub use crate::stdlib::SDL_APP_WILLENTERBACKGROUND;
pub use crate::stdlib::SDL_APP_WILLENTERFOREGROUND;
pub use crate::stdlib::SDL_AUDIODEVICEADDED;
pub use crate::stdlib::SDL_AUDIODEVICEREMOVED;
pub use crate::stdlib::SDL_CLIPBOARDUPDATE;
pub use crate::stdlib::SDL_CONTROLLERAXISMOTION;
pub use crate::stdlib::SDL_CONTROLLERBUTTONDOWN;
pub use crate::stdlib::SDL_CONTROLLERBUTTONUP;
pub use crate::stdlib::SDL_CONTROLLERDEVICEADDED;
pub use crate::stdlib::SDL_CONTROLLERDEVICEREMAPPED;
pub use crate::stdlib::SDL_CONTROLLERDEVICEREMOVED;
pub use crate::stdlib::SDL_DISPLAYEVENT;
pub use crate::stdlib::SDL_DOLLARGESTURE;
pub use crate::stdlib::SDL_DOLLARRECORD;
pub use crate::stdlib::SDL_DROPBEGIN;
pub use crate::stdlib::SDL_DROPCOMPLETE;
pub use crate::stdlib::SDL_DROPFILE;
pub use crate::stdlib::SDL_DROPTEXT;
pub use crate::stdlib::SDL_FINGERDOWN;
pub use crate::stdlib::SDL_FINGERMOTION;
pub use crate::stdlib::SDL_FINGERUP;
pub use crate::stdlib::SDL_FIRSTEVENT;
pub use crate::stdlib::SDL_GETEVENT;
pub use crate::stdlib::SDL_JOYAXISMOTION;
pub use crate::stdlib::SDL_JOYBALLMOTION;
pub use crate::stdlib::SDL_JOYBUTTONDOWN;
pub use crate::stdlib::SDL_JOYBUTTONUP;
pub use crate::stdlib::SDL_JOYDEVICEADDED;
pub use crate::stdlib::SDL_JOYDEVICEREMOVED;
pub use crate::stdlib::SDL_JOYHATMOTION;
pub use crate::stdlib::SDL_KEYDOWN;
pub use crate::stdlib::SDL_KEYMAPCHANGED;
pub use crate::stdlib::SDL_KEYUP;
pub use crate::stdlib::SDL_LASTEVENT;
pub use crate::stdlib::SDL_MOUSEBUTTONDOWN;
pub use crate::stdlib::SDL_MOUSEBUTTONUP;
pub use crate::stdlib::SDL_MOUSEMOTION;
pub use crate::stdlib::SDL_MOUSEWHEEL;
pub use crate::stdlib::SDL_MULTIGESTURE;
pub use crate::stdlib::SDL_PEEKEVENT;
pub use crate::stdlib::SDL_QUIT;
pub use crate::stdlib::SDL_RENDER_DEVICE_RESET;
pub use crate::stdlib::SDL_RENDER_TARGETS_RESET;
pub use crate::stdlib::SDL_SENSORUPDATE;
pub use crate::stdlib::SDL_SYSWMEVENT;
pub use crate::stdlib::SDL_TEXTEDITING;
pub use crate::stdlib::SDL_TEXTINPUT;
pub use crate::stdlib::SDL_USEREVENT;
pub use crate::stdlib::SDL_WINDOWEVENT;
pub use crate::tr_types_h::glDriverType_t;
pub use crate::tr_types_h::glHardwareType_t;
pub use crate::tr_types_h::glconfig_t;
pub use crate::tr_types_h::textureCompression_t;
pub use crate::tr_types_h::GLDRV_ICD;
pub use crate::tr_types_h::GLDRV_STANDALONE;
pub use crate::tr_types_h::GLDRV_VOODOO;
pub use crate::tr_types_h::GLHW_3DFX_2D3D;
pub use crate::tr_types_h::GLHW_GENERIC;
pub use crate::tr_types_h::GLHW_PERMEDIA2;
pub use crate::tr_types_h::GLHW_RAGEPRO;
pub use crate::tr_types_h::GLHW_RIVA128;
pub use crate::tr_types_h::TC_NONE;
pub use crate::tr_types_h::TC_S3TC;
pub use crate::tr_types_h::TC_S3TC_ARB;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_151 {
    pub buttons: [qboolean; 16],
    pub oldaxes: u32,
    pub oldaaxes: [i32; 16],
    pub oldhats: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_152 {
    pub key: keyNum_t,
    pub character: i32,
}

pub type consoleKey_t = consoleKey_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct consoleKey_s {
    pub type_0: C2RustUnnamed_153,
    pub u: C2RustUnnamed_152,
}

pub type C2RustUnnamed_153 = u32;

pub const CHARACTER: C2RustUnnamed_153 = 1;

pub const QUAKE_KEY: C2RustUnnamed_153 = 0;
/*
===========================================================================
Copyright (C) 1999-2005 Id Software, Inc.

This file is part of Quake III Arena source code.

Quake III Arena source code is free software; you can redistribute it
and/or modify it under the terms of the GNU General Public License as
published by the Free Software Foundation; either version 2 of the License,
or (at your option) any later version.

Quake III Arena source code is distributed in the hope that it will be
useful, but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with Quake III Arena source code; if not, write to the Free Software
Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
===========================================================================
*/

static mut in_keyboardDebug: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;

static mut gamepad: *mut SDL_GameController =
    0 as *const SDL_GameController as *mut SDL_GameController;

static mut stick: *mut SDL_Joystick = 0 as *const SDL_Joystick as *mut SDL_Joystick;

static mut mouseAvailable: qboolean = qfalse;

static mut mouseActive: qboolean = qfalse;

static mut in_mouse: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;

static mut in_nograb: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;

static mut in_joystick: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;

static mut in_joystickThreshold: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;

static mut in_joystickNo: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;

static mut in_joystickUseAnalog: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;

static mut vidRestartTime: i32 = 0 as i32;

static mut in_eventTime: i32 = 0 as i32;

static mut SDL_window: *mut SDL_Window = 0 as *const SDL_Window as *mut SDL_Window;
/*
===============
IN_PrintKey
===============
*/

unsafe extern "C" fn IN_PrintKey(
    mut keysym: *const SDL_Keysym,
    mut key: keyNum_t,
    mut down: qboolean,
) {
    if down as u64 != 0 {
        Com_Printf(b"+ \x00" as *const u8 as *const libc::c_char);
    } else {
        Com_Printf(b"  \x00" as *const u8 as *const libc::c_char);
    }
    Com_Printf(
        b"Scancode: 0x%02x(%s) Sym: 0x%02x(%s)\x00" as *const u8 as *const libc::c_char,
        (*keysym).scancode as u32,
        SDL_GetScancodeName((*keysym).scancode),
        (*keysym).sym,
        SDL_GetKeyName((*keysym).sym),
    );
    if (*keysym).mod_0 as i32 & KMOD_LSHIFT as i32 != 0 {
        Com_Printf(b" KMOD_LSHIFT\x00" as *const u8 as *const libc::c_char);
    }
    if (*keysym).mod_0 as i32 & KMOD_RSHIFT as i32 != 0 {
        Com_Printf(b" KMOD_RSHIFT\x00" as *const u8 as *const libc::c_char);
    }
    if (*keysym).mod_0 as i32 & KMOD_LCTRL as i32 != 0 {
        Com_Printf(b" KMOD_LCTRL\x00" as *const u8 as *const libc::c_char);
    }
    if (*keysym).mod_0 as i32 & KMOD_RCTRL as i32 != 0 {
        Com_Printf(b" KMOD_RCTRL\x00" as *const u8 as *const libc::c_char);
    }
    if (*keysym).mod_0 as i32 & KMOD_LALT as i32 != 0 {
        Com_Printf(b" KMOD_LALT\x00" as *const u8 as *const libc::c_char);
    }
    if (*keysym).mod_0 as i32 & KMOD_RALT as i32 != 0 {
        Com_Printf(b" KMOD_RALT\x00" as *const u8 as *const libc::c_char);
    }
    if (*keysym).mod_0 as i32 & KMOD_LGUI as i32 != 0 {
        Com_Printf(b" KMOD_LGUI\x00" as *const u8 as *const libc::c_char);
    }
    if (*keysym).mod_0 as i32 & KMOD_RGUI as i32 != 0 {
        Com_Printf(b" KMOD_RGUI\x00" as *const u8 as *const libc::c_char);
    }
    if (*keysym).mod_0 as i32 & KMOD_NUM as i32 != 0 {
        Com_Printf(b" KMOD_NUM\x00" as *const u8 as *const libc::c_char);
    }
    if (*keysym).mod_0 as i32 & KMOD_CAPS as i32 != 0 {
        Com_Printf(b" KMOD_CAPS\x00" as *const u8 as *const libc::c_char);
    }
    if (*keysym).mod_0 as i32 & KMOD_MODE as i32 != 0 {
        Com_Printf(b" KMOD_MODE\x00" as *const u8 as *const libc::c_char);
    }
    if (*keysym).mod_0 as i32 & KMOD_RESERVED as i32 != 0 {
        Com_Printf(b" KMOD_RESERVED\x00" as *const u8 as *const libc::c_char);
    }
    Com_Printf(
        b" Q:0x%02x(%s)\n\x00" as *const u8 as *const libc::c_char,
        key as u32,
        Key_KeynumToString(key as i32),
    );
}
/*
===============
IN_IsConsoleKey

TODO: If the SDL_Scancode situation improves, use it instead of
      both of these methods
===============
*/

unsafe extern "C" fn IN_IsConsoleKey(mut key: keyNum_t, mut character: i32) -> qboolean {
    static mut consoleKeys: [consoleKey_t; 16] = [consoleKey_t {
        type_0: QUAKE_KEY,
        u: C2RustUnnamed_152 { key: 0 as keyNum_t },
    }; 16];
    static mut numConsoleKeys: i32 = 0 as i32;
    let mut i: i32 = 0;
    // Only parse the variable when it changes
    if (*cl_consoleKeys).modified as u64 != 0 {
        let mut text_p: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
        (*cl_consoleKeys).modified = qfalse;
        text_p = (*cl_consoleKeys).string;
        numConsoleKeys = 0 as i32;
        while numConsoleKeys < 16 as i32 {
            let mut c: *mut consoleKey_t =
                &mut *consoleKeys.as_mut_ptr().offset(numConsoleKeys as isize) as *mut consoleKey_t;
            let mut charCode: i32 = 0 as i32;
            token = COM_Parse(&mut text_p);
            if *token.offset(0 as i32 as isize) == 0 {
                break;
            }
            charCode = Com_HexStrToInt(token);
            if charCode > 0 as i32 {
                (*c).type_0 = CHARACTER;
                (*c).u.character = charCode
            } else {
                (*c).type_0 = QUAKE_KEY;
                (*c).u.key = Key_StringToKeynum(token) as keyNum_t;
                // 0 isn't a key
                if (*c).u.key as u32 <= 0 as i32 as u32 {
                    continue;
                }
            }
            numConsoleKeys += 1
        }
    }
    // If the character is the same as the key, prefer the character
    if key as u32 == character as u32 {
        key = 0 as keyNum_t
    }
    i = 0 as i32;
    while i < numConsoleKeys {
        let mut c_0: *mut consoleKey_t =
            &mut *consoleKeys.as_mut_ptr().offset(i as isize) as *mut consoleKey_t;
        match (*c_0).type_0 as u32 {
            0 => {
                if key as u32 != 0 && (*c_0).u.key as u32 == key as u32 {
                    return qtrue;
                }
            }
            1 => {
                if (*c_0).u.character == character {
                    return qtrue;
                }
            }
            _ => {}
        }
        i += 1
    }
    return qfalse;
}
/*
===============
IN_TranslateSDLToQ3Key
===============
*/

unsafe extern "C" fn IN_TranslateSDLToQ3Key(
    mut keysym: *mut SDL_Keysym,
    mut down: qboolean,
) -> keyNum_t {
    let mut key: keyNum_t = 0 as keyNum_t;
    if (*keysym).scancode as u32 >= SDL_SCANCODE_1 as i32 as u32
        && (*keysym).scancode as u32 <= SDL_SCANCODE_0 as i32 as u32
    {
        // Always map the number keys as such even if they actually map
        // to other characters (eg, "1" is "&" on an AZERTY keyboard).
        // This is required for SDL before 2.0.6, except on Windows
        // which already had this behavior.
        if (*keysym).scancode as u32 == SDL_SCANCODE_0 as i32 as u32 {
            key = '0' as i32 as keyNum_t
        } else {
            key = ('1' as i32 as u32)
                .wrapping_add((*keysym).scancode as u32)
                .wrapping_sub(SDL_SCANCODE_1 as i32 as u32) as keyNum_t
        }
    } else if (*keysym).sym >= SDLK_SPACE as i32 && (*keysym).sym < SDLK_DELETE as i32 {
        // These happen to match the ASCII chars
        key = (*keysym).sym as keyNum_t
    } else {
        match (*keysym).sym {
            1073741899 => key = K_PGUP,
            1073741921 => key = K_KP_PGUP,
            1073741902 => key = K_PGDN,
            1073741915 => key = K_KP_PGDN,
            1073741919 => key = K_KP_HOME,
            1073741898 => key = K_HOME,
            1073741913 => key = K_KP_END,
            1073741901 => key = K_END,
            1073741916 => key = K_KP_LEFTARROW,
            1073741904 => key = K_LEFTARROW,
            1073741918 => key = K_KP_RIGHTARROW,
            1073741903 => key = K_RIGHTARROW,
            1073741914 => key = K_KP_DOWNARROW,
            1073741905 => key = K_DOWNARROW,
            1073741920 => key = K_KP_UPARROW,
            1073741906 => key = K_UPARROW,
            27 => key = K_ESCAPE,
            1073741912 => key = K_KP_ENTER,
            13 => key = K_ENTER,
            9 => key = K_TAB,
            1073741882 => key = K_F1,
            1073741883 => key = K_F2,
            1073741884 => key = K_F3,
            1073741885 => key = K_F4,
            1073741886 => key = K_F5,
            1073741887 => key = K_F6,
            1073741888 => key = K_F7,
            1073741889 => key = K_F8,
            1073741890 => key = K_F9,
            1073741891 => key = K_F10,
            1073741892 => key = K_F11,
            1073741893 => key = K_F12,
            1073741928 => key = K_F13,
            1073741929 => key = K_F14,
            1073741930 => key = K_F15,
            8 => key = K_BACKSPACE,
            1073741923 => key = K_KP_DEL,
            127 => key = K_DEL,
            1073741896 => key = K_PAUSE,
            1073742049 | 1073742053 => key = K_SHIFT,
            1073742048 | 1073742052 => key = K_CTRL,
            1073742055 | 1073742051 => key = K_SUPER,
            1073742054 | 1073742050 => key = K_ALT,
            1073741917 => key = K_KP_5,
            1073741897 => key = K_INS,
            1073741922 => key = K_KP_INS,
            1073741909 => key = K_KP_STAR,
            1073741911 => key = K_KP_PLUS,
            1073741910 => key = K_KP_MINUS,
            1073741908 => key = K_KP_SLASH,
            1073742081 => key = K_MODE,
            1073741941 => key = K_HELP,
            1073741894 => key = K_PRINT,
            1073741978 => key = K_SYSREQ,
            1073741942 => key = K_MENU,
            1073741925 => key = K_MENU,
            1073741926 => key = K_POWER,
            1073741946 => key = K_UNDO,
            1073741895 => key = K_SCROLLOCK,
            1073741907 => key = K_KP_NUMLOCK,
            1073741881 => key = K_CAPSLOCK,
            _ => {
                if (*keysym).sym & (1 as i32) << 30 as i32 == 0
                    && (*keysym).scancode as u32 <= 95 as i32 as u32
                {
                    // Map Unicode characters to 95 world keys using the key's scan code.
                    // FIXME: There aren't enough world keys to cover all the scancodes.
                    // Maybe create a map of scancode to quake key at start up and on
                    // key map change; allocate world key numbers as needed similar
                    // to SDL 1.2.
                    key = (K_WORLD_0 as i32 + (*keysym).scancode as i32) as keyNum_t
                }
            }
        }
    }
    if (*in_keyboardDebug).integer != 0 {
        IN_PrintKey(keysym, key, down);
    }
    if IN_IsConsoleKey(key, 0 as i32) as u64 != 0 {
        // Console keys can't be bound or generate characters
        key = K_CONSOLE
    }
    return key;
}
/*
===============
IN_GobbleMotionEvents
===============
*/

unsafe extern "C" fn IN_GobbleMotionEvents() {
    let mut dummy: [SDL_Event; 1] = [SDL_Event { type_0: 0 }; 1];
    let mut val: i32 = 0 as i32;
    // Gobble any mouse motion events
    SDL_PumpEvents();
    loop {
        val = SDL_PeepEvents(
            dummy.as_mut_ptr(),
            1 as i32,
            SDL_GETEVENT,
            SDL_MOUSEMOTION as i32 as Uint32,
            SDL_MOUSEMOTION as i32 as Uint32,
        );
        if !(val > 0 as i32) {
            break;
        }
    }
    if val < 0 as i32 {
        Com_Printf(
            b"IN_GobbleMotionEvents failed: %s\n\x00" as *const u8 as *const libc::c_char,
            crate::stdlib::SDL_GetError(),
        );
    };
}
/*
===============
IN_ActivateMouse
===============
*/

unsafe extern "C" fn IN_ActivateMouse(mut isFullscreen: qboolean) {
    if mouseAvailable as u64 == 0 || crate::stdlib::SDL_WasInit(0x20 as u32) == 0 {
        return;
    }
    if mouseActive as u64 == 0 {
        crate::stdlib::SDL_SetRelativeMouseMode(SDL_TRUE);
        SDL_SetWindowGrab(SDL_window, SDL_TRUE);
        IN_GobbleMotionEvents();
    }
    // in_nograb makes no sense in fullscreen mode
    if isFullscreen as u64 == 0 {
        if (*in_nograb).modified as u32 != 0 || mouseActive as u64 == 0 {
            if (*in_nograb).integer != 0 {
                crate::stdlib::SDL_SetRelativeMouseMode(SDL_FALSE);
                SDL_SetWindowGrab(SDL_window, SDL_FALSE);
            } else {
                crate::stdlib::SDL_SetRelativeMouseMode(SDL_TRUE);
                SDL_SetWindowGrab(SDL_window, SDL_TRUE);
            }
            (*in_nograb).modified = qfalse
        }
    }
    mouseActive = qtrue;
}
/*
===============
IN_DeactivateMouse
===============
*/

unsafe extern "C" fn IN_DeactivateMouse(mut isFullscreen: qboolean) {
    if crate::stdlib::SDL_WasInit(0x20 as u32) == 0 {
        return;
    }
    // Always show the cursor when the mouse is disabled,
    // but not when fullscreen
    if isFullscreen as u64 == 0 {
        crate::stdlib::SDL_ShowCursor(SDL_TRUE as i32);
    }
    if mouseAvailable as u64 == 0 {
        return;
    }
    if mouseActive as u64 != 0 {
        IN_GobbleMotionEvents();
        SDL_SetWindowGrab(SDL_window, SDL_FALSE);
        crate::stdlib::SDL_SetRelativeMouseMode(SDL_FALSE);
        // Don't warp the mouse unless the cursor is within the window
        if SDL_GetWindowFlags(SDL_window) & SDL_WINDOW_MOUSE_FOCUS as i32 as u32 != 0 {
            crate::stdlib::SDL_WarpMouseInWindow(
                SDL_window,
                cls.glconfig.vidWidth / 2 as i32,
                cls.glconfig.vidHeight / 2 as i32,
            );
        }
        mouseActive = qfalse
    };
}
// We translate axes movement into keypresses

static mut joy_keys: [i32; 16] = [
    K_LEFTARROW as i32,
    K_RIGHTARROW as i32,
    K_UPARROW as i32,
    K_DOWNARROW as i32,
    K_JOY17 as i32,
    K_JOY18 as i32,
    K_JOY19 as i32,
    K_JOY20 as i32,
    K_JOY21 as i32,
    K_JOY22 as i32,
    K_JOY23 as i32,
    K_JOY24 as i32,
    K_JOY25 as i32,
    K_JOY26 as i32,
    K_JOY27 as i32,
    K_JOY28 as i32,
];
// translate hat events into keypresses
// the 4 highest buttons are used for the first hat ...

static mut hat_keys: [i32; 16] = [
    K_JOY29 as i32,
    K_JOY30 as i32,
    K_JOY31 as i32,
    K_JOY32 as i32,
    K_JOY25 as i32,
    K_JOY26 as i32,
    K_JOY27 as i32,
    K_JOY28 as i32,
    K_JOY21 as i32,
    K_JOY22 as i32,
    K_JOY23 as i32,
    K_JOY24 as i32,
    K_JOY17 as i32,
    K_JOY18 as i32,
    K_JOY19 as i32,
    K_JOY20 as i32,
];
#[no_mangle]

pub static mut stick_state: C2RustUnnamed_151 = C2RustUnnamed_151 {
    buttons: [qfalse; 16],
    oldaxes: 0,
    oldaaxes: [0; 16],
    oldhats: 0,
};
/*
===============
IN_InitJoystick
===============
*/

unsafe extern "C" fn IN_InitJoystick() {
    let mut i: i32 = 0 as i32;
    let mut total: i32 = 0 as i32;
    let mut buf: [libc::c_char; 16384] =
        *::std::mem::transmute::<&[u8; 16384],
                                 &mut [libc::c_char; 16384]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
    if !gamepad.is_null() {
        SDL_GameControllerClose(gamepad);
    }
    if !stick.is_null() {
        SDL_JoystickClose(stick);
    }
    stick = 0 as *mut SDL_Joystick;
    gamepad = 0 as *mut SDL_GameController;
    crate::stdlib::memset(
        &mut stick_state as *mut C2RustUnnamed_151 as *mut libc::c_void,
        '\u{0}' as i32,
        ::std::mem::size_of::<C2RustUnnamed_151>() as libc::c_ulong,
    );
    // SDL 2.0.4 requires SDL_INIT_JOYSTICK to be initialized separately from
    // SDL_INIT_GAMECONTROLLER for SDL_JoystickOpen() to work correctly,
    // despite https://wiki.libsdl.org/SDL_Init (retrieved 2016-08-16)
    // indicating SDL_INIT_JOYSTICK should be initialized automatically.
    if crate::stdlib::SDL_WasInit(0x200 as u32) == 0 {
        Com_DPrintf(
            b"Calling SDL_Init(SDL_INIT_JOYSTICK)...\n\x00" as *const u8 as *const libc::c_char,
        );
        if crate::stdlib::SDL_Init(0x200 as u32) != 0 as i32 {
            Com_DPrintf(
                b"SDL_Init(SDL_INIT_JOYSTICK) failed: %s\n\x00" as *const u8 as *const libc::c_char,
                crate::stdlib::SDL_GetError(),
            );
            return;
        }
        Com_DPrintf(
            b"SDL_Init(SDL_INIT_JOYSTICK) passed.\n\x00" as *const u8 as *const libc::c_char,
        );
    }
    if crate::stdlib::SDL_WasInit(0x2000 as u32) == 0 {
        Com_DPrintf(
            b"Calling SDL_Init(SDL_INIT_GAMECONTROLLER)...\n\x00" as *const u8
                as *const libc::c_char,
        );
        if crate::stdlib::SDL_Init(0x2000 as u32) != 0 as i32 {
            Com_DPrintf(
                b"SDL_Init(SDL_INIT_GAMECONTROLLER) failed: %s\n\x00" as *const u8
                    as *const libc::c_char,
                crate::stdlib::SDL_GetError(),
            );
            return;
        }
        Com_DPrintf(
            b"SDL_Init(SDL_INIT_GAMECONTROLLER) passed.\n\x00" as *const u8 as *const libc::c_char,
        );
    }
    total = SDL_NumJoysticks();
    Com_DPrintf(
        b"%d possible joysticks\n\x00" as *const u8 as *const libc::c_char,
        total,
    );
    // Print list and build cvar to allow ui to select joystick.
    i = 0 as i32;
    while i < total {
        Q_strcat(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 16384]>() as libc::c_ulong as i32,
            SDL_JoystickNameForIndex(i),
        );
        Q_strcat(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 16384]>() as libc::c_ulong as i32,
            b"\n\x00" as *const u8 as *const libc::c_char,
        );
        i += 1
    }

    Cvar_Get(
        b"in_availableJoysticks\x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
        0x40 as i32,
    ) as *mut cvar_s;
    if (*in_joystick).integer == 0 {
        Com_DPrintf(b"Joystick is not active.\n\x00" as *const u8 as *const libc::c_char);
        crate::stdlib::SDL_QuitSubSystem(0x2000 as u32);
        return;
    }
    in_joystickNo = Cvar_Get(
        b"in_joystickNo\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x1 as i32,
    ) as *mut cvar_s;
    if (*in_joystickNo).integer < 0 as i32 || (*in_joystickNo).integer >= total {
        Cvar_Set(
            b"in_joystickNo\x00" as *const u8 as *const libc::c_char,
            b"0\x00" as *const u8 as *const libc::c_char,
        );
    }
    in_joystickUseAnalog = Cvar_Get(
        b"in_joystickUseAnalog\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x1 as i32,
    ) as *mut cvar_s;
    stick = SDL_JoystickOpen((*in_joystickNo).integer);
    if stick.is_null() {
        Com_DPrintf(
            b"No joystick opened: %s\n\x00" as *const u8 as *const libc::c_char,
            crate::stdlib::SDL_GetError(),
        );
        return;
    }
    if SDL_IsGameController((*in_joystickNo).integer) as u64 != 0 {
        gamepad = SDL_GameControllerOpen((*in_joystickNo).integer)
    }
    Com_DPrintf(
        b"Joystick %d opened\n\x00" as *const u8 as *const libc::c_char,
        (*in_joystickNo).integer,
    );
    Com_DPrintf(
        b"Name:       %s\n\x00" as *const u8 as *const libc::c_char,
        SDL_JoystickNameForIndex((*in_joystickNo).integer),
    );
    Com_DPrintf(
        b"Axes:       %d\n\x00" as *const u8 as *const libc::c_char,
        SDL_JoystickNumAxes(stick),
    );
    Com_DPrintf(
        b"Hats:       %d\n\x00" as *const u8 as *const libc::c_char,
        SDL_JoystickNumHats(stick),
    );
    Com_DPrintf(
        b"Buttons:    %d\n\x00" as *const u8 as *const libc::c_char,
        SDL_JoystickNumButtons(stick),
    );
    Com_DPrintf(
        b"Balls:      %d\n\x00" as *const u8 as *const libc::c_char,
        SDL_JoystickNumBalls(stick),
    );
    Com_DPrintf(
        b"Use Analog: %s\n\x00" as *const u8 as *const libc::c_char,
        if (*in_joystickUseAnalog).integer != 0 {
            b"Yes\x00" as *const u8 as *const libc::c_char
        } else {
            b"No\x00" as *const u8 as *const libc::c_char
        },
    );
    Com_DPrintf(
        b"Is gamepad: %s\n\x00" as *const u8 as *const libc::c_char,
        if !gamepad.is_null() {
            b"Yes\x00" as *const u8 as *const libc::c_char
        } else {
            b"No\x00" as *const u8 as *const libc::c_char
        },
    );
    SDL_JoystickEventState(-(1 as i32));
    SDL_GameControllerEventState(-(1 as i32));
}
/*
===============
IN_ShutdownJoystick
===============
*/

unsafe extern "C" fn IN_ShutdownJoystick() {
    if crate::stdlib::SDL_WasInit(0x2000 as u32) == 0 {
        return;
    }
    if crate::stdlib::SDL_WasInit(0x200 as u32) == 0 {
        return;
    }
    if !gamepad.is_null() {
        SDL_GameControllerClose(gamepad);
        gamepad = 0 as *mut SDL_GameController
    }
    if !stick.is_null() {
        SDL_JoystickClose(stick);
        stick = 0 as *mut SDL_Joystick
    }
    crate::stdlib::SDL_QuitSubSystem(0x2000 as u32);
    crate::stdlib::SDL_QuitSubSystem(0x200 as u32);
}

unsafe extern "C" fn KeyToAxisAndSign(
    mut keynum: i32,
    mut outAxis: *mut i32,
    mut outSign: *mut i32,
) -> qboolean {
    let mut bind: *mut libc::c_char = 0 as *mut libc::c_char;
    if keynum == 0 {
        return qfalse;
    }
    bind = Key_GetBinding(keynum);
    if bind.is_null() || *bind as i32 != '+' as i32 {
        return qfalse;
    }
    *outSign = 0 as i32;
    if Q_stricmp(bind, b"+forward\x00" as *const u8 as *const libc::c_char) == 0 as i32 {
        *outAxis = (*j_forward_axis).integer;
        *outSign = if (*j_forward).value > 0.0f32 {
            1 as i32
        } else {
            -(1 as i32)
        }
    } else if Q_stricmp(bind, b"+back\x00" as *const u8 as *const libc::c_char) == 0 as i32 {
        *outAxis = (*j_forward_axis).integer;
        *outSign = if (*j_forward).value > 0.0f32 {
            -(1 as i32)
        } else {
            1 as i32
        }
    } else if Q_stricmp(bind, b"+moveleft\x00" as *const u8 as *const libc::c_char) == 0 as i32 {
        *outAxis = (*j_side_axis).integer;
        *outSign = if (*j_side).value > 0.0f32 {
            -(1 as i32)
        } else {
            1 as i32
        }
    } else if Q_stricmp(bind, b"+moveright\x00" as *const u8 as *const libc::c_char) == 0 as i32 {
        *outAxis = (*j_side_axis).integer;
        *outSign = if (*j_side).value > 0.0f32 {
            1 as i32
        } else {
            -(1 as i32)
        }
    } else if Q_stricmp(bind, b"+lookup\x00" as *const u8 as *const libc::c_char) == 0 as i32 {
        *outAxis = (*j_pitch_axis).integer;
        *outSign = if (*j_pitch).value > 0.0f32 {
            -(1 as i32)
        } else {
            1 as i32
        }
    } else if Q_stricmp(bind, b"+lookdown\x00" as *const u8 as *const libc::c_char) == 0 as i32 {
        *outAxis = (*j_pitch_axis).integer;
        *outSign = if (*j_pitch).value > 0.0f32 {
            1 as i32
        } else {
            -(1 as i32)
        }
    } else if Q_stricmp(bind, b"+left\x00" as *const u8 as *const libc::c_char) == 0 as i32 {
        *outAxis = (*j_yaw_axis).integer;
        *outSign = if (*j_yaw).value > 0.0f32 {
            1 as i32
        } else {
            -(1 as i32)
        }
    } else if Q_stricmp(bind, b"+right\x00" as *const u8 as *const libc::c_char) == 0 as i32 {
        *outAxis = (*j_yaw_axis).integer;
        *outSign = if (*j_yaw).value > 0.0f32 {
            -(1 as i32)
        } else {
            1 as i32
        }
    } else if Q_stricmp(bind, b"+moveup\x00" as *const u8 as *const libc::c_char) == 0 as i32 {
        *outAxis = (*j_up_axis).integer;
        *outSign = if (*j_up).value > 0.0f32 {
            1 as i32
        } else {
            -(1 as i32)
        }
    } else if Q_stricmp(bind, b"+movedown\x00" as *const u8 as *const libc::c_char) == 0 as i32 {
        *outAxis = (*j_up_axis).integer;
        *outSign = if (*j_up).value > 0.0f32 {
            -(1 as i32)
        } else {
            1 as i32
        }
    }
    return (*outSign != 0 as i32) as i32 as qboolean;
}
/*
===============
IN_GamepadMove
===============
*/

unsafe extern "C" fn IN_GamepadMove() {
    let mut i: i32 = 0;
    let mut translatedAxes: [i32; 16] = [0; 16];
    let mut translatedAxesSet: [qboolean; 16] = [qfalse; 16];
    SDL_GameControllerUpdate();
    // check buttons
    i = 0 as i32;
    while i < SDL_CONTROLLER_BUTTON_MAX as i32 {
        let mut pressed: qboolean = SDL_GameControllerGetButton(
            gamepad,
            (SDL_CONTROLLER_BUTTON_A as i32 + i) as SDL_GameControllerButton,
        ) as qboolean;
        if pressed as u32 != stick_state.buttons[i as usize] as u32 {
            Com_QueueEvent(
                in_eventTime,
                SE_KEY,
                K_PAD0_A as i32 + i,
                pressed as i32,
                0 as i32,
                0 as *mut libc::c_void,
            );
            stick_state.buttons[i as usize] = pressed
        }
        i += 1
    }
    // must defer translated axes until all real axes are processed
    // must be done this way to prevent a later mapped axis from zeroing out a previous one
    if (*in_joystickUseAnalog).integer != 0 {
        i = 0 as i32;
        while i < 16 as i32 {
            translatedAxes[i as usize] = 0 as i32;
            translatedAxesSet[i as usize] = qfalse;
            i += 1
        }
    }
    // check axes
    i = 0 as i32;
    while i < SDL_CONTROLLER_AXIS_MAX as i32 {
        let mut axis: i32 = SDL_GameControllerGetAxis(
            gamepad,
            (SDL_CONTROLLER_AXIS_LEFTX as i32 + i) as SDL_GameControllerAxis,
        ) as i32;
        let mut oldAxis: i32 = stick_state.oldaaxes[i as usize];
        // Smoothly ramp from dead zone to maximum value
        let mut f: f32 = (libc::abs(axis) as f32 / 32767.0f32 - (*in_joystickThreshold).value)
            / (1.0f32 - (*in_joystickThreshold).value);
        if f < 0.0f32 {
            f = 0.0f32
        }
        axis = (32767 as i32 as f32 * (if axis < 0 as i32 { -f } else { f })) as i32;
        if axis != oldAxis {
            let negMap: [i32; 6] = [
                K_PAD0_LEFTSTICK_LEFT as i32,
                K_PAD0_LEFTSTICK_UP as i32,
                K_PAD0_RIGHTSTICK_LEFT as i32,
                K_PAD0_RIGHTSTICK_UP as i32,
                0 as i32,
                0 as i32,
            ];
            let posMap: [i32; 6] = [
                K_PAD0_LEFTSTICK_RIGHT as i32,
                K_PAD0_LEFTSTICK_DOWN as i32,
                K_PAD0_RIGHTSTICK_RIGHT as i32,
                K_PAD0_RIGHTSTICK_DOWN as i32,
                K_PAD0_LEFTTRIGGER as i32,
                K_PAD0_RIGHTTRIGGER as i32,
            ];
            let mut posAnalog: qboolean = qfalse;
            let mut negAnalog: qboolean = qfalse;
            let mut negKey: i32 = negMap[i as usize];
            let mut posKey: i32 = posMap[i as usize];
            if (*in_joystickUseAnalog).integer != 0 {
                let mut posAxis: i32 = 0 as i32;
                let mut posSign: i32 = 0 as i32;
                let mut negAxis: i32 = 0 as i32;
                let mut negSign: i32 = 0 as i32;
                // get axes and axes signs for keys if available
                posAnalog = KeyToAxisAndSign(posKey, &mut posAxis, &mut posSign);
                negAnalog = KeyToAxisAndSign(negKey, &mut negAxis, &mut negSign);
                // positive to negative/neutral -> keyup if axis hasn't yet been set
                if posAnalog as u32 != 0
                    && translatedAxesSet[posAxis as usize] as u64 == 0
                    && oldAxis > 0 as i32
                    && axis <= 0 as i32
                {
                    translatedAxes[posAxis as usize] = 0 as i32;
                    translatedAxesSet[posAxis as usize] = qtrue
                }
                // negative to positive/neutral -> keyup if axis hasn't yet been set
                if negAnalog as u32 != 0
                    && translatedAxesSet[negAxis as usize] as u64 == 0
                    && oldAxis < 0 as i32
                    && axis >= 0 as i32
                {
                    translatedAxes[negAxis as usize] = 0 as i32;
                    translatedAxesSet[negAxis as usize] = qtrue
                }
                // negative/neutral to positive -> keydown
                if posAnalog as u32 != 0 && axis > 0 as i32 {
                    translatedAxes[posAxis as usize] = axis * posSign;
                    translatedAxesSet[posAxis as usize] = qtrue
                }
                // positive/neutral to negative -> keydown
                if negAnalog as u32 != 0 && axis < 0 as i32 {
                    translatedAxes[negAxis as usize] = -axis * negSign;
                    translatedAxesSet[negAxis as usize] = qtrue
                }
            }
            // keyups first so they get overridden by keydowns later
            // positive to negative/neutral -> keyup
            if posAnalog as u64 == 0 && posKey != 0 && oldAxis > 0 as i32 && axis <= 0 as i32 {
                Com_QueueEvent(
                    in_eventTime,
                    SE_KEY,
                    posKey,
                    qfalse as i32,
                    0 as i32,
                    0 as *mut libc::c_void,
                );
            }
            // negative to positive/neutral -> keyup
            if negAnalog as u64 == 0 && negKey != 0 && oldAxis < 0 as i32 && axis >= 0 as i32 {
                Com_QueueEvent(
                    in_eventTime,
                    SE_KEY,
                    negKey,
                    qfalse as i32,
                    0 as i32,
                    0 as *mut libc::c_void,
                );
            }
            // negative/neutral to positive -> keydown
            if posAnalog as u64 == 0 && posKey != 0 && oldAxis <= 0 as i32 && axis > 0 as i32 {
                Com_QueueEvent(
                    in_eventTime,
                    SE_KEY,
                    posKey,
                    qtrue as i32,
                    0 as i32,
                    0 as *mut libc::c_void,
                );
            }
            // positive/neutral to negative -> keydown
            if negAnalog as u64 == 0 && negKey != 0 && oldAxis >= 0 as i32 && axis < 0 as i32 {
                Com_QueueEvent(
                    in_eventTime,
                    SE_KEY,
                    negKey,
                    qtrue as i32,
                    0 as i32,
                    0 as *mut libc::c_void,
                );
            }
            stick_state.oldaaxes[i as usize] = axis
        }
        i += 1
    }
    // set translated axes
    if (*in_joystickUseAnalog).integer != 0 {
        i = 0 as i32;
        while i < 16 as i32 {
            if translatedAxesSet[i as usize] as u64 != 0 {
                Com_QueueEvent(
                    in_eventTime,
                    SE_JOYSTICK_AXIS,
                    i,
                    translatedAxes[i as usize],
                    0 as i32,
                    0 as *mut libc::c_void,
                );
            }
            i += 1
        }
    };
}
/*
===============
IN_JoyMove
===============
*/

unsafe extern "C" fn IN_JoyMove() {
    let mut axes: u32 = 0 as i32 as u32;
    let mut hats: u32 = 0 as i32 as u32;
    let mut total: i32 = 0 as i32;
    let mut i: i32 = 0 as i32;
    if !gamepad.is_null() {
        IN_GamepadMove();
        return;
    }
    if stick.is_null() {
        return;
    }
    SDL_JoystickUpdate();
    // update the ball state.
    total = SDL_JoystickNumBalls(stick);
    if total > 0 as i32 {
        let mut balldx: i32 = 0 as i32;
        let mut balldy: i32 = 0 as i32;
        i = 0 as i32;
        while i < total {
            let mut dx: i32 = 0 as i32;
            let mut dy: i32 = 0 as i32;
            SDL_JoystickGetBall(stick, i, &mut dx, &mut dy);
            balldx += dx;
            balldy += dy;
            i += 1
        }
        if balldx != 0 || balldy != 0 {
            // !!! FIXME: is this good for stick balls, or just mice?
            // Scale like the mouse input...
            if libc::abs(balldx) > 1 as i32 {
                balldx *= 2 as i32
            }
            if libc::abs(balldy) > 1 as i32 {
                balldy *= 2 as i32
            }
            Com_QueueEvent(
                in_eventTime,
                SE_MOUSE,
                balldx,
                balldy,
                0 as i32,
                0 as *mut libc::c_void,
            );
        }
    }
    // now query the stick buttons...
    total = SDL_JoystickNumButtons(stick);
    if total > 0 as i32 {
        if total as libc::c_ulong
            > (::std::mem::size_of::<[qboolean; 16]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<qboolean>() as libc::c_ulong)
        {
            total = (::std::mem::size_of::<[qboolean; 16]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<qboolean>() as libc::c_ulong)
                as i32
        }
        i = 0 as i32;
        while i < total {
            let mut pressed: qboolean =
                (SDL_JoystickGetButton(stick, i) as i32 != 0 as i32) as i32 as qboolean;
            if pressed as u32 != stick_state.buttons[i as usize] as u32 {
                Com_QueueEvent(
                    in_eventTime,
                    SE_KEY,
                    K_JOY1 as i32 + i,
                    pressed as i32,
                    0 as i32,
                    0 as *mut libc::c_void,
                );
                stick_state.buttons[i as usize] = pressed
            }
            i += 1
        }
    }
    // look at the hats...
    total = SDL_JoystickNumHats(stick);
    if total > 0 as i32 {
        if total > 4 as i32 {
            total = 4 as i32
        }
        i = 0 as i32;
        while i < total {
            *(&mut hats as *mut u32 as *mut Uint8).offset(i as isize) =
                SDL_JoystickGetHat(stick, i);
            i += 1
        }
    }
    // update hat state
    if hats != stick_state.oldhats {
        i = 0 as i32;
        while i < 4 as i32 {
            if *(&mut hats as *mut u32 as *mut Uint8).offset(i as isize) as i32
                != *(&mut stick_state.oldhats as *mut u32 as *mut Uint8).offset(i as isize) as i32
            {
                // release event
                match *(&mut stick_state.oldhats as *mut u32 as *mut Uint8).offset(i as isize)
                    as i32
                {
                    1 => {
                        Com_QueueEvent(
                            in_eventTime,
                            SE_KEY,
                            hat_keys[(4 as i32 * i + 0 as i32) as usize],
                            qfalse as i32,
                            0 as i32,
                            0 as *mut libc::c_void,
                        );
                    }
                    2 => {
                        Com_QueueEvent(
                            in_eventTime,
                            SE_KEY,
                            hat_keys[(4 as i32 * i + 1 as i32) as usize],
                            qfalse as i32,
                            0 as i32,
                            0 as *mut libc::c_void,
                        );
                    }
                    4 => {
                        Com_QueueEvent(
                            in_eventTime,
                            SE_KEY,
                            hat_keys[(4 as i32 * i + 2 as i32) as usize],
                            qfalse as i32,
                            0 as i32,
                            0 as *mut libc::c_void,
                        );
                    }
                    8 => {
                        Com_QueueEvent(
                            in_eventTime,
                            SE_KEY,
                            hat_keys[(4 as i32 * i + 3 as i32) as usize],
                            qfalse as i32,
                            0 as i32,
                            0 as *mut libc::c_void,
                        );
                    }
                    3 => {
                        Com_QueueEvent(
                            in_eventTime,
                            SE_KEY,
                            hat_keys[(4 as i32 * i + 0 as i32) as usize],
                            qfalse as i32,
                            0 as i32,
                            0 as *mut libc::c_void,
                        );
                        Com_QueueEvent(
                            in_eventTime,
                            SE_KEY,
                            hat_keys[(4 as i32 * i + 1 as i32) as usize],
                            qfalse as i32,
                            0 as i32,
                            0 as *mut libc::c_void,
                        );
                    }
                    6 => {
                        Com_QueueEvent(
                            in_eventTime,
                            SE_KEY,
                            hat_keys[(4 as i32 * i + 2 as i32) as usize],
                            qfalse as i32,
                            0 as i32,
                            0 as *mut libc::c_void,
                        );
                        Com_QueueEvent(
                            in_eventTime,
                            SE_KEY,
                            hat_keys[(4 as i32 * i + 1 as i32) as usize],
                            qfalse as i32,
                            0 as i32,
                            0 as *mut libc::c_void,
                        );
                    }
                    9 => {
                        Com_QueueEvent(
                            in_eventTime,
                            SE_KEY,
                            hat_keys[(4 as i32 * i + 0 as i32) as usize],
                            qfalse as i32,
                            0 as i32,
                            0 as *mut libc::c_void,
                        );
                        Com_QueueEvent(
                            in_eventTime,
                            SE_KEY,
                            hat_keys[(4 as i32 * i + 3 as i32) as usize],
                            qfalse as i32,
                            0 as i32,
                            0 as *mut libc::c_void,
                        );
                    }
                    12 => {
                        Com_QueueEvent(
                            in_eventTime,
                            SE_KEY,
                            hat_keys[(4 as i32 * i + 2 as i32) as usize],
                            qfalse as i32,
                            0 as i32,
                            0 as *mut libc::c_void,
                        );
                        Com_QueueEvent(
                            in_eventTime,
                            SE_KEY,
                            hat_keys[(4 as i32 * i + 3 as i32) as usize],
                            qfalse as i32,
                            0 as i32,
                            0 as *mut libc::c_void,
                        );
                    }
                    _ => {}
                }
                // press event
                match *(&mut hats as *mut u32 as *mut Uint8).offset(i as isize) as i32 {
                    1 => {
                        Com_QueueEvent(
                            in_eventTime,
                            SE_KEY,
                            hat_keys[(4 as i32 * i + 0 as i32) as usize],
                            qtrue as i32,
                            0 as i32,
                            0 as *mut libc::c_void,
                        );
                    }
                    2 => {
                        Com_QueueEvent(
                            in_eventTime,
                            SE_KEY,
                            hat_keys[(4 as i32 * i + 1 as i32) as usize],
                            qtrue as i32,
                            0 as i32,
                            0 as *mut libc::c_void,
                        );
                    }
                    4 => {
                        Com_QueueEvent(
                            in_eventTime,
                            SE_KEY,
                            hat_keys[(4 as i32 * i + 2 as i32) as usize],
                            qtrue as i32,
                            0 as i32,
                            0 as *mut libc::c_void,
                        );
                    }
                    8 => {
                        Com_QueueEvent(
                            in_eventTime,
                            SE_KEY,
                            hat_keys[(4 as i32 * i + 3 as i32) as usize],
                            qtrue as i32,
                            0 as i32,
                            0 as *mut libc::c_void,
                        );
                    }
                    3 => {
                        Com_QueueEvent(
                            in_eventTime,
                            SE_KEY,
                            hat_keys[(4 as i32 * i + 0 as i32) as usize],
                            qtrue as i32,
                            0 as i32,
                            0 as *mut libc::c_void,
                        );
                        Com_QueueEvent(
                            in_eventTime,
                            SE_KEY,
                            hat_keys[(4 as i32 * i + 1 as i32) as usize],
                            qtrue as i32,
                            0 as i32,
                            0 as *mut libc::c_void,
                        );
                    }
                    6 => {
                        Com_QueueEvent(
                            in_eventTime,
                            SE_KEY,
                            hat_keys[(4 as i32 * i + 2 as i32) as usize],
                            qtrue as i32,
                            0 as i32,
                            0 as *mut libc::c_void,
                        );
                        Com_QueueEvent(
                            in_eventTime,
                            SE_KEY,
                            hat_keys[(4 as i32 * i + 1 as i32) as usize],
                            qtrue as i32,
                            0 as i32,
                            0 as *mut libc::c_void,
                        );
                    }
                    9 => {
                        Com_QueueEvent(
                            in_eventTime,
                            SE_KEY,
                            hat_keys[(4 as i32 * i + 0 as i32) as usize],
                            qtrue as i32,
                            0 as i32,
                            0 as *mut libc::c_void,
                        );
                        Com_QueueEvent(
                            in_eventTime,
                            SE_KEY,
                            hat_keys[(4 as i32 * i + 3 as i32) as usize],
                            qtrue as i32,
                            0 as i32,
                            0 as *mut libc::c_void,
                        );
                    }
                    12 => {
                        Com_QueueEvent(
                            in_eventTime,
                            SE_KEY,
                            hat_keys[(4 as i32 * i + 2 as i32) as usize],
                            qtrue as i32,
                            0 as i32,
                            0 as *mut libc::c_void,
                        );
                        Com_QueueEvent(
                            in_eventTime,
                            SE_KEY,
                            hat_keys[(4 as i32 * i + 3 as i32) as usize],
                            qtrue as i32,
                            0 as i32,
                            0 as *mut libc::c_void,
                        );
                    }
                    _ => {}
                }
            }
            i += 1
        }
    }
    // save hat state
    stick_state.oldhats = hats;
    // finally, look at the axes...
    total = SDL_JoystickNumAxes(stick);
    if total > 0 as i32 {
        if (*in_joystickUseAnalog).integer != 0 {
            if total > 16 as i32 {
                total = 16 as i32
            }
            i = 0 as i32;
            while i < total {
                let mut axis: Sint16 = SDL_JoystickGetAxis(stick, i);
                let mut f: f32 = libc::abs(axis as i32) as f32 / 32767.0f32;
                if f < (*in_joystickThreshold).value {
                    axis = 0 as i32 as Sint16
                }
                if axis as i32 != stick_state.oldaaxes[i as usize] {
                    Com_QueueEvent(
                        in_eventTime,
                        SE_JOYSTICK_AXIS,
                        i,
                        axis as i32,
                        0 as i32,
                        0 as *mut libc::c_void,
                    );
                    stick_state.oldaaxes[i as usize] = axis as i32
                }
                i += 1
            }
        } else {
            if total > 16 as i32 {
                total = 16 as i32
            }
            i = 0 as i32;
            while i < total {
                let mut axis_0: Sint16 = SDL_JoystickGetAxis(stick, i);
                let mut f_0: f32 = axis_0 as f32 / 32767.0f32;
                if f_0 < -(*in_joystickThreshold).value {
                    axes |= ((1 as i32) << i * 2 as i32) as u32
                } else if f_0 > (*in_joystickThreshold).value {
                    axes |= ((1 as i32) << i * 2 as i32 + 1 as i32) as u32
                }
                i += 1
            }
        }
    }
    /* Time to update axes state based on old vs. new. */
    if axes != stick_state.oldaxes {
        i = 0 as i32;
        while i < 16 as i32 {
            if axes & ((1 as i32) << i) as u32 != 0
                && stick_state.oldaxes & ((1 as i32) << i) as u32 == 0
            {
                Com_QueueEvent(
                    in_eventTime,
                    SE_KEY,
                    joy_keys[i as usize],
                    qtrue as i32,
                    0 as i32,
                    0 as *mut libc::c_void,
                );
            }
            if axes & ((1 as i32) << i) as u32 == 0
                && stick_state.oldaxes & ((1 as i32) << i) as u32 != 0
            {
                Com_QueueEvent(
                    in_eventTime,
                    SE_KEY,
                    joy_keys[i as usize],
                    qfalse as i32,
                    0 as i32,
                    0 as *mut libc::c_void,
                );
            }
            i += 1
        }
    }
    /* Save for future generations. */
    stick_state.oldaxes = axes;
}
/*
===============
IN_ProcessEvents
===============
*/

unsafe extern "C" fn IN_ProcessEvents() {
    let mut e: SDL_Event = SDL_Event { type_0: 0 };
    let mut key: keyNum_t = 0 as keyNum_t;
    static mut lastKeyDown: keyNum_t = 0 as keyNum_t;
    if crate::stdlib::SDL_WasInit(0x20 as u32) == 0 {
        return;
    }
    while SDL_PollEvent(&mut e) != 0 {
        match e.type_0 {
            768 => {
                if !(e.key.repeat as i32 != 0 && Key_GetCatcher() == 0 as i32) {
                    key = IN_TranslateSDLToQ3Key(&mut e.key.keysym, qtrue);
                    if key as u64 != 0 {
                        Com_QueueEvent(
                            in_eventTime,
                            SE_KEY,
                            key as i32,
                            qtrue as i32,
                            0 as i32,
                            0 as *mut libc::c_void,
                        );
                    }
                    if key as u32 == K_BACKSPACE as i32 as u32 {
                        Com_QueueEvent(
                            in_eventTime,
                            SE_CHAR,
                            'h' as i32 - 'a' as i32 + 1 as i32,
                            0 as i32,
                            0 as i32,
                            0 as *mut libc::c_void,
                        );
                    } else if keys[K_CTRL as i32 as usize].down as u32 != 0
                        && key as u32 >= 'a' as i32 as u32
                        && key as u32 <= 'z' as i32 as u32
                    {
                        Com_QueueEvent(
                            in_eventTime,
                            SE_CHAR,
                            (key as u32)
                                .wrapping_sub('a' as i32 as u32)
                                .wrapping_add(1 as i32 as u32) as i32,
                            0 as i32,
                            0 as i32,
                            0 as *mut libc::c_void,
                        );
                    }
                    lastKeyDown = key
                }
            }
            769 => {
                key = IN_TranslateSDLToQ3Key(&mut e.key.keysym, qfalse);
                if key as u64 != 0 {
                    Com_QueueEvent(
                        in_eventTime,
                        SE_KEY,
                        key as i32,
                        qfalse as i32,
                        0 as i32,
                        0 as *mut libc::c_void,
                    );
                }
                lastKeyDown = 0 as keyNum_t
            }
            771 => {
                if lastKeyDown as u32 != K_CONSOLE as i32 as u32 {
                    let mut c: *mut libc::c_char = e.text.text.as_mut_ptr();
                    // Quick and dirty UTF-8 to UTF-32 conversion
                    while *c != 0 {
                        let mut utf32: i32 = 0 as i32;
                        if *c as i32 & 0x80 as i32 == 0 as i32 {
                            let fresh0 = c;
                            c = c.offset(1);
                            utf32 = *fresh0 as i32
                        } else if *c as i32 & 0xe0 as i32 == 0xc0 as i32 {
                            // 110x xxxx
                            let fresh1 = c;
                            c = c.offset(1);
                            utf32 |= (*fresh1 as i32 & 0x1f as i32) << 6 as i32;
                            let fresh2 = c;
                            c = c.offset(1);
                            utf32 |= *fresh2 as i32 & 0x3f as i32
                        } else if *c as i32 & 0xf0 as i32 == 0xe0 as i32 {
                            // 1110 xxxx
                            let fresh3 = c;
                            c = c.offset(1);
                            utf32 |= (*fresh3 as i32 & 0xf as i32) << 12 as i32;
                            let fresh4 = c;
                            c = c.offset(1);
                            utf32 |= (*fresh4 as i32 & 0x3f as i32) << 6 as i32;
                            let fresh5 = c;
                            c = c.offset(1);
                            utf32 |= *fresh5 as i32 & 0x3f as i32
                        } else if *c as i32 & 0xf8 as i32 == 0xf0 as i32 {
                            // 1111 0xxx
                            let fresh6 = c;
                            c = c.offset(1);
                            utf32 |= (*fresh6 as i32 & 0x7 as i32) << 18 as i32;
                            let fresh7 = c;
                            c = c.offset(1);
                            utf32 |= (*fresh7 as i32 & 0x3f as i32) << 12 as i32;
                            let fresh8 = c;
                            c = c.offset(1);
                            utf32 |= (*fresh8 as i32 & 0x3f as i32) << 6 as i32;
                            let fresh9 = c;
                            c = c.offset(1);
                            utf32 |= *fresh9 as i32 & 0x3f as i32
                        } else {
                            Com_DPrintf(
                                b"Unrecognised UTF-8 lead byte: 0x%x\n\x00" as *const u8
                                    as *const libc::c_char,
                                *c as u32,
                            );
                            c = c.offset(1)
                        }
                        if utf32 != 0 as i32 {
                            if IN_IsConsoleKey(0 as keyNum_t, utf32) as u64 != 0 {
                                Com_QueueEvent(
                                    in_eventTime,
                                    SE_KEY,
                                    K_CONSOLE as i32,
                                    qtrue as i32,
                                    0 as i32,
                                    0 as *mut libc::c_void,
                                );
                                Com_QueueEvent(
                                    in_eventTime,
                                    SE_KEY,
                                    K_CONSOLE as i32,
                                    qfalse as i32,
                                    0 as i32,
                                    0 as *mut libc::c_void,
                                );
                            } else {
                                Com_QueueEvent(
                                    in_eventTime,
                                    SE_CHAR,
                                    utf32,
                                    0 as i32,
                                    0 as i32,
                                    0 as *mut libc::c_void,
                                );
                            }
                        }
                    }
                }
            }
            1024 => {
                if mouseActive as u64 != 0 {
                    if !(e.motion.xrel == 0 && e.motion.yrel == 0) {
                        Com_QueueEvent(
                            in_eventTime,
                            SE_MOUSE,
                            e.motion.xrel,
                            e.motion.yrel,
                            0 as i32,
                            0 as *mut libc::c_void,
                        );
                    }
                }
            }
            1025 | 1026 => {
                let mut b: i32 = 0;
                match e.button.button as i32 {
                    1 => b = K_MOUSE1 as i32,
                    2 => b = K_MOUSE3 as i32,
                    3 => b = K_MOUSE2 as i32,
                    4 => b = K_MOUSE4 as i32,
                    5 => b = K_MOUSE5 as i32,
                    _ => {
                        b = K_AUX1 as i32
                            + (e.button.button as i32 - 5 as i32 + 1 as i32) % 16 as i32
                    }
                }
                Com_QueueEvent(
                    in_eventTime,
                    SE_KEY,
                    b,
                    if e.type_0 == SDL_MOUSEBUTTONDOWN as i32 as u32 {
                        qtrue as i32
                    } else {
                        qfalse as i32
                    },
                    0 as i32,
                    0 as *mut libc::c_void,
                );
            }
            1027 => {
                if e.wheel.y > 0 as i32 {
                    Com_QueueEvent(
                        in_eventTime,
                        SE_KEY,
                        K_MWHEELUP as i32,
                        qtrue as i32,
                        0 as i32,
                        0 as *mut libc::c_void,
                    );
                    Com_QueueEvent(
                        in_eventTime,
                        SE_KEY,
                        K_MWHEELUP as i32,
                        qfalse as i32,
                        0 as i32,
                        0 as *mut libc::c_void,
                    );
                } else if e.wheel.y < 0 as i32 {
                    Com_QueueEvent(
                        in_eventTime,
                        SE_KEY,
                        K_MWHEELDOWN as i32,
                        qtrue as i32,
                        0 as i32,
                        0 as *mut libc::c_void,
                    );
                    Com_QueueEvent(
                        in_eventTime,
                        SE_KEY,
                        K_MWHEELDOWN as i32,
                        qfalse as i32,
                        0 as i32,
                        0 as *mut libc::c_void,
                    );
                }
            }
            1619 | 1620 => {
                if (*in_joystick).integer != 0 {
                    IN_InitJoystick();
                }
            }
            256 => {
                Cbuf_ExecuteText(
                    EXEC_NOW as i32,
                    b"quit Closed window\n\x00" as *const u8 as *const libc::c_char,
                );
            }
            512 => {
                match e.window.event as i32 {
                    5 => {
                        let mut width: i32 = 0;
                        let mut height: i32 = 0;
                        width = e.window.data1;
                        height = e.window.data2;
                        // ignore this event on fullscreen
                        if !(cls.glconfig.isFullscreen as u64 != 0) {
                            // check if size actually changed
                            if !(cls.glconfig.vidWidth == width && cls.glconfig.vidHeight == height)
                            {
                                Cvar_SetValue(
                                    b"r_customwidth\x00" as *const u8 as *const libc::c_char,
                                    width as f32,
                                );
                                Cvar_SetValue(
                                    b"r_customheight\x00" as *const u8 as *const libc::c_char,
                                    height as f32,
                                );
                                Cvar_Set(
                                    b"r_mode\x00" as *const u8 as *const libc::c_char,
                                    b"-1\x00" as *const u8 as *const libc::c_char,
                                );
                                // Wait until user stops dragging for 1 second, so
                                // we aren't constantly recreating the GL context while
                                // he tries to drag...
                                vidRestartTime = Sys_Milliseconds() + 1000 as i32
                            }
                        }
                    }
                    7 => {
                        Cvar_SetValue(
                            b"com_minimized\x00" as *const u8 as *const libc::c_char,
                            1 as i32 as f32,
                        );
                    }
                    9 | 8 => {
                        Cvar_SetValue(
                            b"com_minimized\x00" as *const u8 as *const libc::c_char,
                            0 as i32 as f32,
                        );
                    }
                    13 => {
                        Cvar_SetValue(
                            b"com_unfocused\x00" as *const u8 as *const libc::c_char,
                            1 as i32 as f32,
                        );
                    }
                    12 => {
                        Cvar_SetValue(
                            b"com_unfocused\x00" as *const u8 as *const libc::c_char,
                            0 as i32 as f32,
                        );
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
/*
===============
IN_Frame
===============
*/
#[no_mangle]

pub unsafe extern "C" fn IN_Frame() {
    let mut loading: qboolean = qfalse;
    IN_JoyMove();
    // If not DISCONNECTED (main menu) or ACTIVE (in game), we're loading
    loading = (clc.state as u32 != CA_DISCONNECTED as i32 as u32
        && clc.state as u32 != CA_ACTIVE as i32 as u32) as i32 as qboolean;
    // update isFullscreen since it might of changed since the last vid_restart
    cls.glconfig.isFullscreen =
        (Cvar_VariableIntegerValue(b"r_fullscreen\x00" as *const u8 as *const libc::c_char)
            != 0 as i32) as i32 as qboolean;
    if cls.glconfig.isFullscreen as u64 == 0 && Key_GetCatcher() & 0x1 as i32 != 0 {
        // Console is down in windowed mode
        IN_DeactivateMouse(cls.glconfig.isFullscreen);
    } else if cls.glconfig.isFullscreen as u64 == 0 && loading as u32 != 0 {
        // Loading in windowed mode
        IN_DeactivateMouse(cls.glconfig.isFullscreen);
    } else if SDL_GetWindowFlags(SDL_window) & SDL_WINDOW_INPUT_FOCUS as i32 as u32 == 0 {
        // Window not got focus
        IN_DeactivateMouse(cls.glconfig.isFullscreen);
    } else {
        IN_ActivateMouse(cls.glconfig.isFullscreen);
    }
    IN_ProcessEvents();
    // Set event time for next frame to earliest possible time an event could happen
    in_eventTime = Sys_Milliseconds();
    // In case we had to delay actual restart of video system
    if vidRestartTime != 0 as i32 && vidRestartTime < Sys_Milliseconds() {
        vidRestartTime = 0 as i32;
        Cbuf_AddText(b"vid_restart\n\x00" as *const u8 as *const libc::c_char);
    };
}
/*
===============
IN_Init
===============
*/
#[no_mangle]

pub unsafe extern "C" fn IN_Init(mut windowData: *mut libc::c_void) {
    let mut appState: i32 = 0;
    if crate::stdlib::SDL_WasInit(0x20 as u32) == 0 {
        Com_Error(
            ERR_FATAL as i32,
            b"IN_Init called before SDL_Init( SDL_INIT_VIDEO )\x00" as *const u8
                as *const libc::c_char,
        );
    }
    SDL_window = windowData as *mut SDL_Window;
    Com_DPrintf(
        b"\n------- Input Initialization -------\n\x00" as *const u8 as *const libc::c_char,
    );
    in_keyboardDebug = Cvar_Get(
        b"in_keyboardDebug\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x1 as i32,
    ) as *mut cvar_s;
    // mouse variables
    in_mouse = Cvar_Get(
        b"in_mouse\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x1 as i32,
    ) as *mut cvar_s;
    in_nograb = Cvar_Get(
        b"in_nograb\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x1 as i32,
    ) as *mut cvar_s;
    in_joystick = Cvar_Get(
        b"in_joystick\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x20 as i32,
    ) as *mut cvar_s;
    in_joystickThreshold = Cvar_Get(
        b"joy_threshold\x00" as *const u8 as *const libc::c_char,
        b"0.15\x00" as *const u8 as *const libc::c_char,
        0x1 as i32,
    ) as *mut cvar_s;
    SDL_StartTextInput();
    mouseAvailable = ((*in_mouse).value != 0 as i32 as f32) as i32 as qboolean;
    IN_DeactivateMouse(
        (Cvar_VariableIntegerValue(b"r_fullscreen\x00" as *const u8 as *const libc::c_char)
            != 0 as i32) as i32 as qboolean,
    );
    appState = SDL_GetWindowFlags(SDL_window) as i32;
    Cvar_SetValue(
        b"com_unfocused\x00" as *const u8 as *const libc::c_char,
        (appState & SDL_WINDOW_INPUT_FOCUS as i32 == 0) as i32 as f32,
    );
    Cvar_SetValue(
        b"com_minimized\x00" as *const u8 as *const libc::c_char,
        (appState & SDL_WINDOW_MINIMIZED as i32) as f32,
    );
    IN_InitJoystick();
    Com_DPrintf(b"------------------------------------\n\x00" as *const u8 as *const libc::c_char);
}
/*
===============
IN_Shutdown
===============
*/
#[no_mangle]

pub unsafe extern "C" fn IN_Shutdown() {
    SDL_StopTextInput();
    IN_DeactivateMouse(
        (Cvar_VariableIntegerValue(b"r_fullscreen\x00" as *const u8 as *const libc::c_char)
            != 0 as i32) as i32 as qboolean,
    );
    mouseAvailable = qfalse;
    IN_ShutdownJoystick();
    SDL_window = 0 as *mut SDL_Window;
}
/*
===========================================================================
Copyright (C) 1999-2005 Id Software, Inc.

This file is part of Quake III Arena source code.

Quake III Arena source code is free software; you can redistribute it
and/or modify it under the terms of the GNU General Public License as
published by the Free Software Foundation; either version 2 of the License,
or (at your option) any later version.

Quake III Arena source code is distributed in the hope that it will be
useful, but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with Quake III Arena source code; if not, write to the Free Software
Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
===========================================================================
*/
// qcommon.h -- definitions common between client and server, but not game.or ref modules
//Ignore __attribute__ on non-gcc platforms
//#define	PRE_RELEASE_DEMO
//============================================================================
//
// msg.c
//
// if false, do a Com_Error
// set to true if the buffer size failed (with allowoverflow set)
// set to true if the buffer size failed (with allowoverflow set)
// for bitwise reads and writes
// TTimo
// copy a msg_t in case we need to store it as is for a bit
// (as I needed this to keep an msg_t from a static var for later use)
// sets data buffer as MSG_Init does prior to do the copy
//============================================================================
/*
==============================================================

NET

==============================================================
*/
// if this flag is set, always attempt ipv6 connections instead of ipv4 if a v6 address is found.
// disables ipv6 multicast support if set.
// number of old messages that must be kept on client and
// server for delta comrpession and ping estimation
// max number of usercmd_t in a packet
// max string commands buffered for restransmit
// an address lookup failed
// maximum length of an IPv6 address string including trailing '\0'
// Needed for IPv6 link-local addresses
// max length of a message, which may
// be fragmented into multiple packets
// ACK window of 48 download chunks. Cannot set this higher, or clients
// will overflow the reliable commands buffer
// 896 byte block chunks
/*
Netchan handles packet fragmentation and out of order / duplicate suppression
*/
// between last packet and previous
// qport value to write when transmitting
// sequencing variables
// incoming fragment assembly buffer
// outgoing fragment buffer
// we need to space out the sending of large fragmented messages
/*
==============================================================

PROTOCOL

==============================================================
*/
// 1.31 - 67
// maintain a list of compatible protocols for demo playing
// NOTE: that stuff only works with two digits protocols
// override on command line, config files etc.
// broadcast scan this many ports after
// PORT_SERVER so a single machine can
// run multiple servers
// the svc_strings[] array in cl_parse.c should mirror this
//
// server to client
//
// [short] [string] only in gamestate messages
// only in gamestate messages
// [string] to be executed by client game module
// [short] size [size bytes]
// new commands, supported only by ioquake3 protocol but not legacy
// not wrapped in USE_VOIP, so this value is reserved.
//
//
// client to server
//
// [[usercmd_t]
// [[usercmd_t]
// [string] message
// new commands, supported only by ioquake3 protocol but not legacy
// not wrapped in USE_VOIP, so this value is reserved.
//
/*
==============================================================

VIRTUAL MACHINE

==============================================================
*/
// module should be bare: "cgame", not "cgame.dll" or "vm/cgame.qvm"
/*
==============================================================

CMD

Command text buffering and command execution

==============================================================
*/
/*

Any number of commands can be added in a frame, from several different sources.
Most commands come from either keybindings or console line input, but entire text
files can be execed.

*/
// allocates an initial text buffer that will grow as needed
// Adds command text at the end of the buffer, does NOT add a final \n
// this can be used in place of either Cbuf_AddText or Cbuf_InsertText
// Pulls off \n terminated lines of text from the command buffer and sends
// them through Cmd_ExecuteString.  Stops when the buffer is empty.
// Normally called once per frame, but may be explicitly invoked.
// Do not call inside a command function, or current args will be destroyed.
//===========================================================================
/*

Command execution takes a null terminated string, breaks it into tokens,
then searches for a command or variable that matches the first token.

*/
// called by the init functions of other parts of the program to
// register commands and functions to call for them.
// The cmd_name is referenced later, so it should not be in temp memory
// if function is NULL, the command will be forwarded to the server
// as a clc_clientCommand instead of executed locally
// don't allow VMs to remove system commands
// callback with each valid string
// The functions that execute commands get their parameters with these
// functions. Cmd_Argv () will return an empty string, not a NULL
// if arg > argc, so string operations are allways safe.
// Takes a null terminated string.  Does not need to be /n terminated.
// breaks the string up into arg tokens.
// Parses a single line of text into arguments and tries to execute it
// as if it was typed at the console
/*
==============================================================

CVAR

==============================================================
*/
/*

cvar_t variables are used to hold scalar or string variables that can be changed
or displayed at the console or prog code as well as accessed directly
in C code.

The user can access cvars from the console in three ways:
r_draworder			prints the current value
r_draworder 0		sets the current value to 0
set r_draworder 0	as above, but creates the cvar if not present

Cvars are restricted from having the same names as commands to keep this
interface from being ambiguous.

The are also occasionally used to communicated information between different
modules of the program.

*/
// creates the variable if it doesn't exist, or returns the existing one
// if it exists, the value will not be changed, but flags will be ORed in
// that allows variables to be unarchived without needing bitflags
// if value is "", the value will not override a previously set value.
// basically a slightly modified Cvar_Get for the interpreted modules
// updates an interpreted modules' version of a cvar
// will create the variable with no flags if it doesn't exist
// same as Cvar_Set, but allows more control over setting of cvar
// sometimes we set variables from an untrusted source: fail if flags & CVAR_PROTECTED
// don't set the cvar immediately
// expands value to a string and calls Cvar_Set/Cvar_SetSafe
// returns 0 if not defined or non numeric
// returns an empty string if not defined
// returns CVAR_NONEXISTENT if cvar doesn't exist or the flags of that particular CVAR.
// callback with each valid string
// reset all testing vars to a safe value
// called by Cmd_ExecuteString when Cmd_Argv(0) doesn't match a known
// command.  Returns true if the command was a variable reference that
// was handled. (print or change)
// writes lines containing "set variable value" for all variables
// with the archive flag set to true.
// returns an info string containing all the cvars that have the given bit set
// in their flags ( CVAR_USERINFO, CVAR_SERVERINFO, CVAR_SYSTEMINFO, etc )
// whenever a cvar is modifed, its flags will be OR'd into this, so
// a single check can determine if any CVAR_USERINFO, CVAR_SERVERINFO,
// etc, variables have been modified since the last check.  The bit
// can then be cleared to allow another change detection.
/*
==============================================================

FILESYSTEM

No stdio calls should be used by any part of the game, because
we need to deal with all sorts of directory and seperator char
issues.
==============================================================
*/
// referenced flags
// these are in loop specific order so don't change the order
// number of id paks that will never be autodownloaded from baseq3/missionpack
// shutdown and restart the filesystem so changes to fs_gamedir can take effect
// directory should not have either a leading or trailing /
// if extension is "/", only subdirectories will be returned
// the returned files will not include any directories or /
// will properly create any needed paths and deal with seperater character issues
// if uniqueFILE is true, then a new FILE will be fopened even if the file
// is found in an already open pak file.  If uniqueFILE is false, you must call
// FS_FCloseFile instead of fclose, otherwise the pak FILE would be improperly closed
// It is generally safe to always set uniqueFILE to true, because the majority of
// file IO goes through FS_ReadFile, which Does The Right Thing already.
// returns 1 if a file is in the PAK file, otherwise -1
// properly handles partial reads and reads from other dlls
// note: you can't just fclose from another DLL, due to MS libc issues
// returns the length of the file
// a null buffer will just return the file length without loading
// as a quick check for existence. -1 length == not present
// A 0 byte will always be appended at the end, so string ops are safe.
// the buffer should be considered read-only, because it may be cached
// for other uses.
// forces flush on files we're writing to.
// frees the memory returned by FS_ReadFile
// writes a complete file, creating any subdirectories needed
// doesn't work for files that are opened from a pack file
// where are we?
// like fprintf
// opens a file for reading, writing, or appending depending on the value of mode
// seek on a file
// Returns a space separated string containing the checksums of all loaded pk3 files.
// Servers with sv_pure set will get this string and pass it to clients.
// Returns a space separated string containing the checksums of all loaded
// AND referenced pk3 files. Servers with sv_pure set will get this string
// back from clients for pure validation
// clears referenced booleans on loaded pk3s
// If the string is empty, all data sources will be allowed.
// If not empty, only pk3 files that match one of the space
// separated checksums will be checked for files, with the
// sole exception of .cfg files.
/*
==============================================================

Edit fields and command line history/completion

==============================================================
*/
/*
==============================================================

MISC

==============================================================
*/
// centralizing the declarations for cl_cdkey
// https://zerowing.idsoftware.com/bugzilla/show_bug.cgi?id=470
// returned by Sys_GetProcessorFeatures
// centralized and cleaned, that's the max string you can send to a Com_Printf / Com_DPrintf (above gets truncated)
// SE_NONE must be zero
// evTime is still valid
// evValue is a key code, evValue2 is the down flag
// evValue is an ascii char
// evValue and evValue2 are relative signed x / y moves
// evValue is an axis number and evValue2 is the current state (-127 to 127)
// evPtr is a char*
// bytes of data pointed to by evPtr, for journaling
// this must be manually freed if not NULL
// will be journaled properly
// checks for and removes command line "+set var arg" constructs
// if match is NULL, all set commands will be executed, otherwise
// only a set with the exact name.  Only used during startup.
// for building release pak files
// both client and server must agree to pause
// com_speeds times
// renderer backend time
/*

--- low memory ----
server vm
server clipmap
---mark---
renderer initialization (shaders, etc)
UI vm
cgame vm
renderer map
renderer models

---free---

temp file loading
--- high memory ---

*/
// NOT 0 filled memory
// returns 0 filled memory
// NOT 0 filled memory only for small allocations
// commandLine should not include the executable name (argv[0])
/*
==============================================================

CLIENT / SERVER SYSTEMS

==============================================================
*/
//
// client interface
//
// the keyboard binding interface must be setup before execing
// config files, but the rest of client startup will happen later
// char events are for field typing, not game control
// do a screen update before starting to load a map
// when the server is going to load a new map, the entire hunk
// will be cleared, so the client must shutdown cgame, ui, and
// the renderer
// adds the current command line as a clc_clientCommand to the client message.
// things like godmode, noclip, etc, are commands directed to the server,
// so when they are typed in at the console, they will need to be forwarded.
// bring up the "need a cd to play" dialog
// dump all memory on an error
// shutdown client
// initialize renderer interface
// start all the client stuff using the hunk
// Restart sound subsystem
// for keyname autocompletion
// for writing the config files
// call before filesystem access
// FIXME: move logging to common?
// AVI files have the start of pixel lines 4 byte-aligned
//
// server interface
//
//
// UI interface
//
//
// input interface
//
/*
===============
IN_Restart
===============
*/
#[no_mangle]

pub unsafe extern "C" fn IN_Restart() {
    IN_ShutdownJoystick();
    IN_Init(SDL_window as *mut libc::c_void);
}
