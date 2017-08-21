using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Threading;
using drone_controller;

namespace drone_controller_windows
{
    class WindowsController
    {
        public static void Poll(short lx, short ly, short rx, short ry)
        {
            Console.WriteLine("Left Stick: x={0}, y={1}, magnitude={2}", lx, ly, Math.Sqrt(lx * lx + ly * ly));
            Console.WriteLine("Right Stick: x={0}, y={1}, magnitude={2}", rx, ry, Math.Sqrt(rx * rx + ry * ry));
        }

        public static void Main(string[] args)
        {
            XBoxController controller = new XBoxController();
            XBoxController.b_Xg
            XBoxController.Poll += Poll;
            Connection conn = new Connection();
            if(conn.ConnectToServer())
            {
                conn.ConnectToDrone();
            }
            
            Thread.Sleep(2000);
            conn.stop();

            Console.ReadKey();
        }
    };
    
}
