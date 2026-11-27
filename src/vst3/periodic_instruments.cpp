// Periodic Instruments - VST3 Plugin
// A VST3 instrument plugin featuring 94 elemental voices (Hydrogen through Plutonium)

#include "public.sdk/source/vst/vstaudioeffect.h"
#include "public.sdk/source/vst/vsteditcontroller.h"
#include "public.sdk/source/main/pluginfactory.h"
#include "pluginterfaces/vst/vstspeaker.h"
#include "pluginterfaces/base/fplatform.h"

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
static const FUID kProcessorUID (0x70657269, 0x6F646963, 0x496E7374, 0x72756D00);
static const FUID kControllerUID (0x70657269, 0x6F646963, 0x496E7374, 0x72430000);

//------------------------------------------------------------------------
// Audio Processor
//------------------------------------------------------------------------
class PeriodicInstrumentsProcessor : public AudioEffect
{
public:
    PeriodicInstrumentsProcessor () : AudioEffect ()
    {
        setControllerClass (kControllerUID);
    }

    tresult PLUGIN_API initialize (FUnknown* context) override
    {
        tresult result = AudioEffect::initialize (context);
        if (result == kResultTrue)
        {
            addAudioInput (STR16 ("Audio Input"), SpeakerArr::kStereo);
            addAudioOutput (STR16 ("Audio Output"), SpeakerArr::kStereo);
        }
        return result;
    }

    tresult PLUGIN_API process (ProcessData& data) override
    {
        if (data.outputs)
        {
            for (int32 i = 0; i < data.numOutputs; ++i)
            {
                for (int32 ch = 0; ch < data.outputs[i].numChannels; ++ch)
                {
                    if (data.outputs[i].channelBuffers32[ch])
                    {
                        memset (data.outputs[i].channelBuffers32[ch], 0,
                                data.numSamples * sizeof (float));
                    }
                }
            }
        }
        return kResultTrue;
    }

    tresult PLUGIN_API setupProcessing (ProcessSetup& setup) override
    {
        if (setup.maxSamplesPerBlock > 4096)
            return kResultFalse;
        if (setup.symbolicSampleSize != kSample32)
            return kResultFalse;
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
// Controller
//------------------------------------------------------------------------
class PeriodicInstrumentsController : public EditController
{
public:
    tresult PLUGIN_API initialize (FUnknown* context) override
    {
        tresult result = EditController::initialize (context);
        if (result == kResultTrue)
        {
            parameters.addParameter (STR16 ("Element Selector"), nullptr, 0, 0.0,
                                    ParameterInfo::kCanAutomate | ParameterInfo::kIsList, 93);
            parameters.addParameter (STR16 ("Mass"), nullptr, 1, 0.5, ParameterInfo::kCanAutomate, 0);
            parameters.addParameter (STR16 ("Reactivity"), nullptr, 2, 0.5, ParameterInfo::kCanAutomate, 0);
            parameters.addParameter (STR16 ("State"), nullptr, 3, 0.5, ParameterInfo::kCanAutomate, 0);
            parameters.addParameter (STR16 ("Radiance"), nullptr, 4, 0.5, ParameterInfo::kCanAutomate, 0);
        }
        return result;
    }

    tresult PLUGIN_API getParameterInfo (int32 paramIndex, ParameterInfo& info) override
    {
        if (paramIndex >= 5)
            return kResultFalse;

        const char* titles[] = {"Element Selector", "Mass", "Reactivity", "State", "Radiance"};
        const char* shortTitles[] = {"Element", "Mass", "React", "State", "Rad"};

        info.title[0] = 0;
        info.shortTitle[0] = 0;
        info.units[0] = 0;

        for (int i = 0; titles[paramIndex][i] && i < 63; i++)
            info.title[i] = titles[paramIndex][i];
        for (int i = 0; shortTitles[paramIndex][i] && i < 63; i++)
            info.shortTitle[i] = shortTitles[paramIndex][i];

        info.id = paramIndex;
        info.stepCount = (paramIndex == 0) ? 93 : 0;
        info.defaultNormalizedValue = (paramIndex == 0) ? 0.0 : 0.5;
        info.unitId = 0;
        info.flags = ParameterInfo::kCanAutomate;
        if (paramIndex == 0)
            info.flags |= ParameterInfo::kIsList;

        return kResultTrue;
    }

    IPlugView* PLUGIN_API createView (const char* name) override
    {
        return nullptr;
    }
};

//------------------------------------------------------------------------
// Factory
//------------------------------------------------------------------------
static FUnknown* createProcessor (void*) { return (IAudioProcessor*)new PeriodicInstrumentsProcessor (); }
static FUnknown* createController (void*) { return (IEditController*)new PeriodicInstrumentsController (); }

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

        static PClassInfo controllerInfo (kControllerUID,
            PClassInfo::kManyInstances,
            kVstComponentControllerClass,
            "Periodic Instruments Controller");
        factory->registerClass (&controllerInfo, createController);
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
void* moduleHandle = nullptr;

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
    // Release the plugin factory
    if (gPluginFactory)
    {
        gPluginFactory->release();
        gPluginFactory = nullptr;
    }
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

} // extern "C"