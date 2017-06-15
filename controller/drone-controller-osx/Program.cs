using System;
using drone_controller;

namespace dronecontrollerosx
{
    class MainClass
    {
        public static void Main(string[] args)
        {
            Connection connection = new Connection();
            if(connection.ConnectToServer()) {
                connection.ConnectToDrone();
            }
        }
    }
}
