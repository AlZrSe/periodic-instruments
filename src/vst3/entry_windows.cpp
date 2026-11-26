// Minimal Windows entry point for VST3 plugin
// Avoids including problematic SDK headers that cause redefinition errors

#include <windows.h>

//------------------------------------------------------------------------
// Module handle
HINSTANCE ghInst = nullptr;

//------------------------------------------------------------------------
// Module entry/exit functions
extern "C" {

__declspec(dllexport) bool InitDll ()
{
    return true;
}

__declspec(dllexport) bool ExitDll ()
{
    return true;
}

} // extern "C"

//------------------------------------------------------------------------
BOOL WINAPI DllMain (HINSTANCE hInst, DWORD dwReason, LPVOID)
{
    if (dwReason == DLL_PROCESS_ATTACH)
    {
        // Store instance handle
        // Could store hInst if needed by plugin
    }
    return TRUE;
}