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
        const string SERVER_ADDRESS = "13.59.251.61";
        //const string SERVER_ADDRESS = "10.0.0.28";

        readonly IPEndPoint LOCAL_ENDPOINT = new IPEndPoint(IPAddress.Parse("0.0.0.0"), 27136);

        readonly IPEndPoint SERVER_ENDPOINT = new IPEndPoint(IPAddress.Parse(SERVER_ADDRESS), SERVER_PORT);

        int drone_port;
        string drone_address;
        IPEndPoint drone_endpoint;

        private UdpClient client;
        public Connection()
        {
            client = new UdpClient(LOCAL_ENDPOINT);
        }

        public bool ConnectToServer()
        {
            //client.Connect(IPAddress.Parse(SERVER_ADDRESS), SERVER_PORT);
            var msg = System.Text.Encoding.UTF8.GetBytes("controller");
            client.Send(msg, msg.Length, SERVER_ENDPOINT);
            Console.WriteLine("Sent: controller");

            Timer timeout = new Timer(20000);
            timeout.Elapsed += (s, e) =>
            {
                client.Close();
            };
            //timeout.Start();
            IPEndPoint ep = new IPEndPoint(IPAddress.Any, 0);
            var responsePong = client.Receive(ref ep);

            Console.WriteLine("Received: " + Encoding.UTF8.GetString(responsePong));

            Console.WriteLine("Getting PI pubic IP...");
        
            var response = client.Receive(ref ep);

            if (response.Length > 0)
            {
                var droneEpStr = Encoding.UTF8.GetString(response);
                var split = droneEpStr.Split(':');
                drone_address = split[0];
                drone_port = Int32.Parse(split[1]);

                Console.WriteLine("Received drone end point: ");
                Console.WriteLine("Addr: " + drone_address);
                Console.WriteLine("Port: " + drone_port);

                drone_endpoint = new IPEndPoint(IPAddress.Parse(drone_address), drone_port);
            } else
            {
                return false;
            }
            return true;
        }

        public bool ConnectToDrone() {
            Console.WriteLine("Lets punch a whole through NAT..");

            Console.WriteLine("Sending first packet. Expected to fail.");
            var msg = Encoding.UTF8.GetBytes("control station");
            client.Send(msg, msg.Length, drone_endpoint);

            Console.WriteLine("Waiting for hole punch.");
            IPEndPoint ep = new IPEndPoint(IPAddress.Any, 0);
            var droneHolePunch = client.Receive(ref ep);
            Console.WriteLine("Got something from: " + ep);
            Console.WriteLine(Encoding.UTF8.GetString(droneHolePunch));

            client.Connect(drone_endpoint);

			msg = Encoding.UTF8.GetBytes("control station");
            client.Send(msg, msg.Length);
			return true;
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
