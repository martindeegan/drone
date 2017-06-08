using System;
using System.Collections.Generic;
using System.Text;

using System.Net.Sockets;
using System.Net;

using System.Timers;
using System.Threading.Tasks;

namespace drone_controller
{
    public class Connection
    {
        const int SERVER_PORT = 7070;
        //const string SERVER_ADDRESS = "13.59.251.61";
        const string SERVER_ADDRESS = "10.0.0.28";

        readonly IPEndPoint SERVER_ENDPOINT = new IPEndPoint(IPAddress.Parse(SERVER_ADDRESS), SERVER_PORT);

        int drone_port;
        string drone_address;
        IPEndPoint drone_endpoint;

        public Connection()
        {
            client = new UdpClient();
        }

        private UdpClient client;

        private bool connectToServer()
        {
            client.Connect(IPAddress.Parse(SERVER_ADDRESS), SERVER_PORT);
            var msg = System.Text.Encoding.ASCII.GetBytes("controller");
            client.Send(msg, msg.Length);

            Timer timeout = new Timer(5000);
            timeout.Elapsed += (s, e) =>
            {
                client.Close();
            };
            timeout.Start();
            IPEndPoint ep = new IPEndPoint(IPAddress.Any, 0);
            var response = client.ReceiveAsync();
			response.Wait();            
            timeout.Stop();

			var buf = response.Result.Buffer;
            if (buf.Length > 0)
            {
                var res = Encoding.ASCII.GetString(buf);
                if (res == "Pong") {
					Console.WriteLine("Successfully pinged server.");
					return true;
                }
            }
            Console.WriteLine("Error: Couldn't ping server");
            return false;
        }

        public bool ConnectToDrone() {
			IPEndPoint ep = new IPEndPoint(IPAddress.Any, 0);
			Timer timeout = new Timer(5000);
			timeout.Elapsed += (s, e) =>
			{
				client.Close();
			};
			timeout.Start();
            var response = client.ReceiveAsync();
            response.Wait();
            timeout.Stop();

            var buf = response.Result.Buffer;
            if(buf.Length > 0) {
                var droneEpStr = Encoding.ASCII.GetString(buf);
                Console.WriteLine("Received drone end point: " + droneEpStr);
                var split = droneEpStr.Split(':');
                drone_address = split[0];
                drone_port = Int32.Parse(split[1]);

                drone_endpoint = new IPEndPoint(IPAddress.Parse(drone_address), drone_port);

                var msg = "controller";
                client.Connect(drone_endpoint);
                client.Send(Encoding.ASCII.GetBytes(msg), msg.Length);
                Console.WriteLine(Encoding.ASCII.GetString(client.Receive(ref ep)));
                
                return true;
            }

            Console.WriteLine("Error: Couldn't connect to drone.");
            return false;
			
        }

        public void stop() 
        {
            client.Close();
        }

    }


    public class MessageTypes
    {
        public const byte CONTROLLER_SERVER_PING = 0;

    };
}
