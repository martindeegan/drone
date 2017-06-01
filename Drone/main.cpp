 // Drone.cpp : Defines the entry point for the console application.
//

#include "stdafx.h"

#include "Input.h"

#include <iostream>

bool running = true;

void Poll(short lx, short ly, short rx, short ry) 
{
  std::cout << "Left Stick: " << lx << ", " << ly << ", magnitude: " << sqrt(lx * lx + ly * ly) << '\n';
  std::cout << "Right Stick: " << rx << ", " << ry << ", magnitude: " << sqrt(rx * rx + ry * ry) << '\n';
}

void Exit(void) 
{
  running = false;
}

int main()
{
  Input input(&Poll);
  input.BindAction(XINPUT_GAMEPAD_DPAD_DOWN, &Exit);

  while (running);


  exit(0);
}

