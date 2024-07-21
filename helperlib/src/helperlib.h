#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

#pragma comment(lib, "ntdll")
#pragma comment(lib, "userenv.lib")
#pragma comment(lib, "ws2_32.lib")

extern "C"
{

  bool los(const double (*point_a)[3], const double (*point_b)[3]);
  void load();
} // extern "C"
