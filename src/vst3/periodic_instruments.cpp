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
// CIDs must match manifest.ttl / moduleinfo.json exactly (raw byte order).
// Define them as byte arrays so they are platform-independent (COM_COMPATIBLE
// changes the interpretation of the four-uint32 FUID constructor).
static const TUID kProcessorTUID = { 'p','e','r','i','o','d','i','c','I','n','s','t','r','u','m', 0 };
static const FUID kProcessorUID = FUID::fromTUID (kProcessorTUID);

static const TUID kControllerTUID = { 'p','e','r','i','o','d','i','c','I','n','s','t','r','C', 0, 0 };
static const FUID kControllerUID = FUID::fromTUID (kControllerTUID);

//------------------------------------------------------------------------
// Audio Processor
//------------------------------------------------------------------------
class PeriodicInstrumentsProcessor : public AudioEffect
{
public:
    PeriodicInstrumentsProcessor () : AudioEffect ()
    {
        addAudioOutput (u"Stereo Out", SpeakerArr::kStereo);
        addEventInput (u"MIDI In", 1);
        setControllerClass (kControllerUID);
    }

    // No process override - use AudioEffect default

    tresult PLUGIN_API setupProcessing (ProcessSetup& setup) override
    {
        return AudioEffect::setupProcessing (setup);
    }

    tresult PLUGIN_API setProcessing (TBool state) override
    {
        return kResultOk;
    }

    tresult PLUGIN_API process (ProcessData& data) override
    {
        return kResultOk;
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

static FUnknown* createController (void*) { return (IEditController*)new Vst::EditController (); }

extern "C" {

#if defined(_MSC_VER)
__declspec(dllexport)
#else
SMTG_EXPORT_SYMBOL __attribute__((used))
#endif
IPluginFactory* PLUGIN_API GetPluginFactory ()
{
    // Use the SDK's global gPluginFactory so its destructor can clear the
    // pointer when the last reference is released. A function-local static
    // would become a dangling pointer after release-to-zero, causing a
    // use-after-free crash on the next GetPluginFactory call.
    if (!Steinberg::gPluginFactory)
    {
        static PFactoryInfo factoryInfo ("AlZrSe",
            "https://github.com/AlZrSe/periodic-instruments",
            "",
            Vst::kDefaultFactoryFlags);
        Steinberg::gPluginFactory = new CPluginFactory (factoryInfo);

        static PClassInfo processorInfo (kProcessorUID,
            PClassInfo::kManyInstances,
            kVstAudioEffectClass,
            "Periodic Instruments");
        Steinberg::gPluginFactory->registerClass (&processorInfo, createProcessor);

        static PClassInfo controllerInfo (kControllerUID,
            PClassInfo::kManyInstances,
            kVstComponentControllerClass,
            "Periodic Instruments");
        Steinberg::gPluginFactory->registerClass (&controllerInfo, createController);
    }
    else
    {
        Steinberg::gPluginFactory->addRef ();
    }

    return Steinberg::gPluginFactory;
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

#if !defined(_MSC_VER)
// Linux/macOS - export ModuleEntry/ModuleExit explicitly
SMTG_EXPORT_SYMBOL __attribute__((used))
bool InitModule ()
{
    return true;
}

SMTG_EXPORT_SYMBOL __attribute__((used))
bool DeinitModule ()
{
    return true;
}

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