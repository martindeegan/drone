using System;
using System.Threading;
using SharpDX.XInput;
using System.Collections.Generic;

namespace drone_controller_windows
{
    public delegate void ButtonPressedDelegate();
    public delegate void TriggerPressedDelegate(byte left, byte right);
    public delegate void JoystickDelegate(short lx, short ly, short rx, short ry);

    class XBoxController
    {
        public const int LEFT_THUMB_DEADZONE = 7849;
        public const int RIGHT_THUMB_DEADZONE = 8689;
        public const int TRIGGER_THREASHOLD = 30;

        public const int REFRESH_RATE = 144;
        public XBoxController()
        {
            controller = null;
            var controllers = new[] { new Controller(UserIndex.One), new Controller(UserIndex.Two), new Controller(UserIndex.Three), new Controller(UserIndex.Four) };
            foreach(var ctrl in controllers)
            {
                if(ctrl.IsConnected)
                {
                    controller = ctrl;
                    Console.WriteLine("Controller found!");
                    Timer timer = new Timer(new TimerCallback(poll), this, 0, 1000/REFRESH_RATE);
                    break;
                }
            }

            if(controller == null)
            {
                Console.WriteLine("Controller not connected.");
            }
        }

        private void InvokeButton(GamepadButtonFlags button)
        {
            if (lastButton != button)
            {
                var evt = buttonMapping[button];
                if(evt != null)
                {
                    evt.Invoke();
                    lastButton = button;
                }
            }
        }

        private void poll(object state)
        {
            if(controller.IsConnected)
            {
                var gp = controller.GetState().Gamepad;

                short lx = gp.LeftThumbX;
                short ly = gp.LeftThumbY;
                short rx = gp.RightThumbX;
                short ry = gp.RightThumbY;
                bool invoke = false;
                if (Math.Sqrt(lx * lx + ly * ly) > LEFT_THUMB_DEADZONE) {
                    invoke = true;
                }
                else
                {
                    lx = 0;
                    ly = 0;
                }
                if (Math.Sqrt(rx * rx + ry * ry) > RIGHT_THUMB_DEADZONE)
                {
                    invoke = true;
                }
                else
                {
                    rx = 0;
                    ry = 0;
                }
                if(invoke)
                {
                    Poll.Invoke(lx, ly, rx, ry);
                }

                byte lt = gp.LeftTrigger;
                byte rt = gp.RightTrigger;
                if(lt > TRIGGER_THREASHOLD || rt > TRIGGER_THREASHOLD)
                {
                    Trigger(lt, rt);
                }
                /*
                switch(gp.Buttons)
                {
                    case GamepadButtonFlags.A:
                        InvokeButton(GamepadButtonFlags.A);
                        break;
                    case GamepadButtonFlags.B:
                        InvokeButton(GamepadButtonFlags.B);
                        break;
                    case GamepadButtonFlags.X:
                        InvokeButton(GamepadButtonFlags.X);
                        break;
                    case GamepadButtonFlags.Y:
                        InvokeButton(GamepadButtonFlags.Y);
                        break;
                    case GamepadButtonFlags.Start:
                        InvokeButton(GamepadButtonFlags.Start);
                        break;
                    case GamepadButtonFlags.Back:
                        InvokeButton(GamepadButtonFlags.Back);
                        break;
                    case GamepadButtonFlags.LeftShoulder:
                        InvokeButton(GamepadButtonFlags.LeftShoulder);
                        break;
                    case GamepadButtonFlags.RightShoulder:
                        InvokeButton(GamepadButtonFlags.RightShoulder);
                        break;
                    case GamepadButtonFlags.LeftThumb:
                        InvokeButton(GamepadButtonFlags.LeftThumb);
                        break;
                    case GamepadButtonFlags.RightThumb:
                        InvokeButton(GamepadButtonFlags.RightThumb);
                        break;
                    case GamepadButtonFlags.DPadDown:
                        InvokeButton(GamepadButtonFlags.DPadDown);
                        break;
                    case GamepadButtonFlags.DPadUp:
                        InvokeButton(GamepadButtonFlags.DPadUp);
                        break;
                    case GamepadButtonFlags.DPadLeft:
                        InvokeButton(GamepadButtonFlags.DPadLeft);
                        break;
                    case GamepadButtonFlags.DPadRight:
                        InvokeButton(GamepadButtonFlags.DPadRight);
                        break;

                }
                */
            }
        }

        public static event JoystickDelegate Poll;
        public static event TriggerPressedDelegate Trigger;
        public static event ButtonPressedDelegate b_A;
        public static event ButtonPressedDelegate b_B;
        public static event ButtonPressedDelegate b_X;
        public static event ButtonPressedDelegate b_Y;
        public static event ButtonPressedDelegate b_Menu;
        public static event ButtonPressedDelegate b_View;
        public static event ButtonPressedDelegate b_LeftBumper;
        public static event ButtonPressedDelegate b_RightBumper;
        public static event ButtonPressedDelegate b_LeftJoystick;
        public static event ButtonPressedDelegate b_RightJoystick;
        public static event ButtonPressedDelegate b_DPadDown;
        public static event ButtonPressedDelegate b_DPadUp;
        public static event ButtonPressedDelegate b_DPadLeft;
        public static event ButtonPressedDelegate b_DPadRight;

        private static Dictionary<GamepadButtonFlags, ButtonPressedDelegate> buttonMapping = new Dictionary<GamepadButtonFlags, ButtonPressedDelegate>()
        {
            {GamepadButtonFlags.A, b_A},
            {GamepadButtonFlags.B, b_B},
            {GamepadButtonFlags.X, b_X},
            {GamepadButtonFlags.Y, b_Y},
            {GamepadButtonFlags.Start, b_Menu},
            {GamepadButtonFlags.Back, b_View},
            {GamepadButtonFlags.LeftShoulder, b_LeftBumper},
            {GamepadButtonFlags.RightShoulder, b_RightBumper},
            {GamepadButtonFlags.LeftThumb, b_LeftJoystick},
            {GamepadButtonFlags.RightThumb, b_RightJoystick},
            {GamepadButtonFlags.DPadDown, b_DPadDown},
            {GamepadButtonFlags.DPadUp, b_DPadUp},
            {GamepadButtonFlags.DPadLeft, b_DPadLeft},
            {GamepadButtonFlags.DPadRight, b_DPadRight},
        };

        private Controller controller;
        private GamepadButtonFlags lastButton;
    }
}
