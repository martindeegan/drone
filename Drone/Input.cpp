#include "stdafx.h"
#include "Input.h"

#include <iostream>

#include "dinput.h"
#include "XInput.h"
#include <wbemidl.h>
#include <oleauto.h>

#pragma comment(lib, "XInput.lib")

Input::Input(std::function<void(short lx, short ly, short rx, short ry)> callback)
  :running(true), PollCallback(callback)
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

void Input::ThreadLoop()
{
  while (running) 
  {
    XINPUT_STATE pState;
    if (XInputGetState(Controller, &pState) == ERROR_SUCCESS) 
    {
      XINPUT_GAMEPAD pGamepad = pState.Gamepad;
      CheckButtonPresses(pGamepad);
      PollCallback(pGamepad.sThumbLX, pGamepad.sThumbLY, pGamepad.sThumbRX, pGamepad.sThumbRY);
    }
    else
    {
      FindController();
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


