// Periodic Instruments - VST3 Plugin
// A VST3 instrument plugin featuring 94 elemental voices (Hydrogen through Plutonium)

#include "public.sdk/source/vst/vstaudioeffect.h"
#include "public.sdk/source/vst/vsteditcontroller.h"
#include "public.sdk/source/main/pluginfactory.h"
#include "pluginterfaces/vst/vstspeaker.h"
#include "pluginterfaces/base/fplatform.h"
#include <cstring>

// Note: base/ustring.h intentionally omitted - it includes ftypes.h which includes
// fstrdefs.h, which contains constexpr function definitions that cause redefinition
// errors on MSVC when included in multiple compilation units.
// We define our own STR16 macro to avoid including fstrdefs.h via ustring.h.

#ifndef STR16
#define STR16(x) L##x
#endif

using namespace Steinberg;
using namespace Steinberg::Vst;

//------------------------------------------------------------------------
// Plugin IDs
//------------------------------------------------------------------------
// CIDs must match manifest.ttl exactly (big-endian byte order as per VST3 spec)
// Use four uint32 constructor with little-endian byte order to match manifest.ttl bytes
// manifest.ttl Processor CID: "706572696f646963496e737472756d00"
// Bytes: 70 65 72 69 6f 64 69 63 49 6e 73 74 72 75 6d 00
// As little-endian uint32: 0x69726570, 0x6369646f, 0x74736e49, 0x006d7572
static const FUID kProcessorUID (0x69726570, 0x6369646f, 0x74736e49, 0x006d7572);
// manifest.ttl Controller CID: "706572696f646963496e737472430000"
// Bytes: 70 65 72 69 6f 64 69 63 49 6e 73 74 72 43 00 00
// As little-endian uint32: 0x69726570, 0x6369646f, 0x74736e49, 0x00437272
static const FUID kControllerUID (0x69726570, 0x6369646f, 0x74736e49, 0x00437272);

//------------------------------------------------------------------------
// Audio Processor
//------------------------------------------------------------------------
class PeriodicInstrumentsProcessor : public AudioEffect
{
public:
    PeriodicInstrumentsProcessor () : AudioEffect ()
    {
        // No controller for now to test loading
    }

    tresult PLUGIN_API initialize (FUnknown* context) override
    {
        // Use AudioEffect::initialize which adds default input and output
        tresult result = AudioEffect::initialize (context);
        return result;
    }

    tresult PLUGIN_API process (ProcessData& data) override
    {
        return kResultTrue;
    }

    tresult PLUGIN_API setupProcessing (ProcessSetup& setup) override
    {
        return AudioEffect::setupProcessing (setup);
    }

    tresult PLUGIN_API canProcessSampleSize (int32 symbolicSampleSize) override
    {
        return (symbolicSampleSize == kSample32) ? kResultTrue : kResultFalse;
    }

    tresult PLUGIN_API setState (IBStream* state) override { return kResultTrue; }
    tresult PLUGIN_API getState (IBStream* state) override { return kResultTrue; }

};

//------------------------------------------------------------------------
// Factory
//------------------------------------------------------------------------
static FUnknown* createProcessor (void*) { return (IAudioProcessor*)new PeriodicInstrumentsProcessor (); }

extern "C" {

#if defined(_MSC_VER)
__declspec(dllexport)
#else
SMTG_EXPORT_SYMBOL __attribute__((used))
#endif
IPluginFactory* PLUGIN_API GetPluginFactory ()
{
    // Use a local static factory to avoid global gPluginFactory symbol issues
    // This avoids the ambiguous symbol conflict with SDK's pluginfactory.h
    static CPluginFactory* factory = nullptr;
    if (!factory)
    {
        static PFactoryInfo factoryInfo ("AlZrSe",
            "https://github.com/AlZrSe/periodic-instruments",
            "",
            Vst::kDefaultFactoryFlags);
        factory = new CPluginFactory (factoryInfo);

        static PClassInfo processorInfo (kProcessorUID,
            PClassInfo::kManyInstances,
            kVstAudioEffectClass,
            "Periodic Instruments");
        factory->registerClass (&processorInfo, createProcessor);
    }
    else
    {
        factory->addRef ();
    }

    return factory;
}

// Module entry/exit functions required by VST3
// On Windows, these are provided by the SDK's dllmain.cpp but we also need to define InitModule/DeinitModule
// On Linux/macOS, we need to export ModuleEntry/ModuleExit explicitly

// Module handle - required by SDK (moduleinit.cpp, vstguieditor.cpp)
// On Windows, this is defined in dllmain.cpp. On Linux/macOS, we define it here.
#if !defined(_MSC_VER)
void* moduleHandle = nullptr;
#endif

// Internal init/deinit functions (must be declared before use)
static int moduleCounter = 0; // counting for ModuleEntry/ModuleExit pairs

bool InitModule ()
{
    // Called when the module is loaded
    return true;
}

bool DeinitModule ()
{
    // Called when the module is unloaded
    return true;
}

#if !defined(_MSC_VER)
// Linux/macOS - export ModuleEntry/ModuleExit explicitly
SMTG_EXPORT_SYMBOL __attribute__((used))
bool ModuleEntry (void* sharedLibraryHandle)
{
    if (++moduleCounter == 1)
    {
        moduleHandle = sharedLibraryHandle;
        return InitModule();
    }
    return true;
}

SMTG_EXPORT_SYMBOL __attribute__((used))
bool ModuleExit (void)
{
    if (--moduleCounter == 0)
    {
        moduleHandle = nullptr;
        return DeinitModule();
    }
    else if (moduleCounter < 0)
        return false;
    return true;
}
#endif
// On Windows, ModuleEntry/ModuleExit are provided by the SDK's dllmain.cpp

} // extern "C"