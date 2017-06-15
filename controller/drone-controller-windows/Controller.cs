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

        static XBoxController()
        {
            Poll += (lx,ly,rx,ry) => { };
            Trigger += (l, r) => { };
            b_A += () => { };
            b_B += () => { };
            b_X += () => { };
            b_Y += () => { };
            b_Start += () => { };
            b_Back += () => { };
            b_LeftShoulder += () => { };
            b_RightShoulder += () => { };
            b_LeftThumb += () => { };
            b_RightThumb += () => { };
            b_DPadDown += () => { };
            b_DPadUp += () => { };
            b_DPadLeft += () => { };
            b_DPadRight += () => { };
        } 

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
                
                switch(gp.Buttons)
                {
                    case GamepadButtonFlags.A:
                        if(lastButton != GamepadButtonFlags.A)
                        {
                            b_A.Invoke();
                            lastButton = GamepadButtonFlags.A;
                        }
                        break;
                    case GamepadButtonFlags.B:
                        if (lastButton != GamepadButtonFlags.B)
                        {
                            b_B.Invoke();
                            lastButton = GamepadButtonFlags.B;
                        }
                        break;
                    case GamepadButtonFlags.X:
                        if (lastButton != GamepadButtonFlags.X)
                        {
                            b_X.Invoke();
                            lastButton = GamepadButtonFlags.X;
                        }
                        break;
                    case GamepadButtonFlags.Y:
                        if (lastButton != GamepadButtonFlags.Y)
                        {
                            b_Y.Invoke();
                            lastButton = GamepadButtonFlags.Y;
                        }
                        break;
                    case GamepadButtonFlags.Start:
                        if (lastButton != GamepadButtonFlags.Start)
                        {
                            b_Start.Invoke();
                            lastButton = GamepadButtonFlags.Start;
                        }
                        break;
                    case GamepadButtonFlags.Back:
                        if (lastButton != GamepadButtonFlags.Back)
                        {
                            b_Back.Invoke();
                            lastButton = GamepadButtonFlags.Back;
                        }
                        break;
                    case GamepadButtonFlags.LeftShoulder:
                        if (lastButton != GamepadButtonFlags.LeftShoulder)
                        {
                            b_LeftShoulder.Invoke();
                            lastButton = GamepadButtonFlags.LeftShoulder;
                        }
                        break;
                    case GamepadButtonFlags.RightShoulder:
                        if (lastButton != GamepadButtonFlags.RightShoulder)
                        {
                            b_RightShoulder.Invoke();
                            lastButton = GamepadButtonFlags.RightShoulder;
                        }
                        break;
                    case GamepadButtonFlags.LeftThumb:
                        if (lastButton != GamepadButtonFlags.LeftThumb)
                        {
                            b_LeftThumb.Invoke();
                            lastButton = GamepadButtonFlags.LeftThumb;
                        }
                        break;
                    case GamepadButtonFlags.RightThumb:
                        if (lastButton != GamepadButtonFlags.RightThumb)
                        {
                            b_RightThumb.Invoke();
                            lastButton = GamepadButtonFlags.RightThumb;
                        }
                        break;
                    case GamepadButtonFlags.DPadDown:
                        if (lastButton != GamepadButtonFlags.DPadDown)
                        {
                            b_DPadDown.Invoke();
                            lastButton = GamepadButtonFlags.DPadDown;
                        }
                        break;
                    case GamepadButtonFlags.DPadUp:
                        if (lastButton != GamepadButtonFlags.DPadUp)
                        {
                            b_DPadUp.Invoke();
                            lastButton = GamepadButtonFlags.DPadUp;
                        }
                        break;
                    case GamepadButtonFlags.DPadLeft:
                        if (lastButton != GamepadButtonFlags.DPadLeft)
                        {
                            b_DPadLeft.Invoke();
                            lastButton = GamepadButtonFlags.DPadLeft;
                        }
                        break;
                    case GamepadButtonFlags.DPadRight:
                        if (lastButton != GamepadButtonFlags.DPadRight)
                        {
                            b_DPadRight.Invoke();
                            lastButton = GamepadButtonFlags.DPadRight;
                        }
                        break;
                }
                
            }
        }

        public static event JoystickDelegate Poll;
        public static event TriggerPressedDelegate Trigger;
        public static event ButtonPressedDelegate b_A;
        public static event ButtonPressedDelegate b_B;
        public static event ButtonPressedDelegate b_X;
        public static event ButtonPressedDelegate b_Y;
        public static event ButtonPressedDelegate b_Start;
        public static event ButtonPressedDelegate b_Back;
        public static event ButtonPressedDelegate b_LeftShoulder;
        public static event ButtonPressedDelegate b_RightShoulder;
        public static event ButtonPressedDelegate b_LeftThumb;
        public static event ButtonPressedDelegate b_RightThumb;
        public static event ButtonPressedDelegate b_DPadDown;
        public static event ButtonPressedDelegate b_DPadUp;
        public static event ButtonPressedDelegate b_DPadLeft;
        public static event ButtonPressedDelegate b_DPadRight;

        private Controller controller;
        private GamepadButtonFlags lastButton;
    }
}
