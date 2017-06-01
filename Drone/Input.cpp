#include "stdafx.h"
#include "Input.h"

#include <iostream>
#include <chrono>

#include "dinput.h"
#include "XInput.h"
#include <wbemidl.h>
#include <oleauto.h>

#pragma comment(lib, "XInput.lib")

#define THUMB_MAX 32767

using namespace std::chrono;

Input::Input(std::function<void(short lx, short ly, short rx, short ry)> callback)
  :running(true), PollCallback(callback), RefreshRate(10)
{
  FindController();

  InputThread = new std::thread(&Input::ThreadLoop, this);
}

Input::~Input()
{
  if (running)
  {
    running = false;
    InputThread->join();
    delete InputThread;
  }
}

DWORD Input::FindController()
{
  XINPUT_STATE pState;
  ZeroMemory(&pState, sizeof(XINPUT_STATE));

  for (DWORD i = 0; i < XUSER_MAX_COUNT; ++i) {
    if (XInputGetState(i, &pState) == ERROR_SUCCESS) {
      Controller = i;
      std::cout << "Found device!\n";
      return ERROR_SUCCESS;
    }
  }

  std::cout << "No controller found!\n";
  return ERROR_DEVICE_NOT_CONNECTED;

}

void Input::CheckButtonPresses(XINPUT_GAMEPAD &gGamepad)
{
  auto it = ButtonMap.find(gGamepad.wButtons);
  if (it != ButtonMap.end())
  {
    (it->second)();
  }
}

float magnitude(short x, short y) {
  return (float)sqrt(x * x + y * y);
}

void Input::ThreadLoop()
{
  auto time = system_clock::now();
  auto lastTick = system_clock::now();
  while (running) 
  {
    time = system_clock::now();
    if (time - lastTick > RefreshRate)
    {
      lastTick = time;
      XINPUT_STATE pState;
      if (XInputGetState(Controller, &pState) == ERROR_SUCCESS)
      {
        XINPUT_GAMEPAD pGamepad = pState.Gamepad;
        CheckButtonPresses(pGamepad);
        short LX = pGamepad.sThumbLX;
        short LY = pGamepad.sThumbLY;
        float lMag = magnitude(LX, LY);
        short RX = pGamepad.sThumbRX;
        short RY = pGamepad.sThumbRY;
        float rMag = magnitude(RX, RY);

        if (lMag < XINPUT_GAMEPAD_LEFT_THUMB_DEADZONE)
        {
          LX = 0;
          LY = 0;
        }
        else if (lMag > THUMB_MAX)
        {
          float ratio = THUMB_MAX / lMag;
          LX = (short)(LX * ratio);
          LY = (short)(LY * ratio);
        }

        if (rMag < XINPUT_GAMEPAD_RIGHT_THUMB_DEADZONE)
        {
          RX = 0;
          RY = 0;
        }
        if (rMag > THUMB_MAX)
        {
          float ratio = THUMB_MAX / rMag;
          RX = (short)(RX * ratio);
          RY = (short)(RY * ratio);
        }

        if (lMag > XINPUT_GAMEPAD_LEFT_THUMB_DEADZONE || rMag > XINPUT_GAMEPAD_RIGHT_THUMB_DEADZONE)
        {
          PollCallback(LX, LY, RX, RY);
        }
      }
      else
      {
        FindController();
      }
    }
  }
}

DWORD Input::GetBatteryCharge()
{
  XINPUT_BATTERY_INFORMATION pBatterInformation;
  if (XInputGetBatteryInformation(Controller, BATTERY_DEVTYPE_GAMEPAD, &pBatterInformation) == ERROR_SUCCESS) {
    return pBatterInformation.BatteryLevel;
  }
  else {
    FindController();
    return ERROR_DEVICE_NOT_CONNECTED;
  }
}

void Input::BindAction(WORD button, std::function<void()> func)
{
  ButtonMap.insert_or_assign(button, func);
}

void Input::Unbind(WORD button)
{
  if (ButtonMap.find(button) != ButtonMap.end()) 
  {
    ButtonMap.erase(button);
  }
}

void Input::SetRefreshRate(milliseconds rate)
{
  RefreshRate = milliseconds(rate);
}

milliseconds Input::GetRefreshRate()
{
  return RefreshRate;
}


