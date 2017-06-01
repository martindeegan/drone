using System;
using System.Collections.Generic;
using System.Text;

using System.Net.Sockets;
using System.Net;
using System.Threading;

namespace drone_controller
{
    class Connection
    {
        const int SERVER_PORT = 6060;
        const string SERVER_ADDRESS = "13.59.251.61";

        Connection()
        {
            client = new UdpClient();
        }

        private UdpClient client;
        private bool running = false;
        private System.Threading.Thread connectionThread;

        public void start()
        {
            connectionThread = new Thread(() =>
            {
                running = true;
                while (!connectToServer() && running)
                {
                    Console.WriteLine("No response from server. Retrying");
                }

                if (running)
                {
                    Console.WriteLine("Connected to server.");
                    runLoop();
                }

                Console.WriteLine("Aborted.");
            });

            connectionThread.Start();
        }

        public void stop()
        {
            running = false;
        }

        private bool connectToServer()
        {
            client.Connect(IPAddress.Parse(SERVER_ADDRESS), SERVER_PORT);
            var msg = System.Text.Encoding.ASCII.GetBytes("Controller");
            client.Send(msg, msg.Length);
            System.Timers.Timer timeout = new System.Timers.Timer();
            timeout.Elapsed += (o, e) => { client.Close(); };
            IPEndPoint ep = new IPEndPoint(IPAddress.Any, 0);
            var response = client.Receive(ref ep);
            if(response.Length > 0)
            {
                return true;
            }
            return false;
        }

        private void runLoop()
        {
            while(running)
            {

            }
        }

        private bool connectToDrone()
        {

            return false;
        }
    }
}
