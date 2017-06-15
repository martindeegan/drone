#pragma once

#include <unordered_map>
#include <functional>
#include <thread>

struct State {
  short LX;
  short LY;
  short RX;
  short RY;
};

class Input
{
public:
  /*Initialize with 'callback' ( void callback(short, short, short, short) )*/
  Input(std::function<void(short lx, short ly, short rx, short ry)> callback);
  ~Input();

  DWORD GetBatteryCharge();

  /*Map 'button' to 'func' ( void func() )*/
  void BindAction(WORD button, std::function<void()> func);
  void Unbind(WORD button);
  
  void SetRefreshRate(std::chrono::milliseconds rate);
  std::chrono::milliseconds GetRefreshRate();

private:

  std::chrono::milliseconds RefreshRate;

  DWORD FindController();
  DWORD Controller;

  void CheckButtonPresses(XINPUT_GAMEPAD &gGamepad);
  std::unordered_map<WORD, std::function<void()>> ButtonMap;

  std::thread *InputThread;
  void ThreadLoop();
  bool running;
  std::function<void(short lx, short ly, short rx, short ry)> PollCallback;

};
