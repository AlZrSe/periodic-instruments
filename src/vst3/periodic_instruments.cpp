// Periodic Instruments - Minimal VST3 Plugin

#include "public.sdk/source/vst/vstaudioeffect.h"
#include "public.sdk/source/main/pluginfactory.h"
#include "base/ustring.h"

using namespace Steinberg;
using namespace Steinberg::Vst;

static const FUID kProcessorUID (0x70657269, 0x6F646963, 0x496E7374, 0x72756D00);

class PeriodicInstrumentsProcessor : public AudioEffect
{
public:
    PeriodicInstrumentsProcessor () : AudioEffect ()
    {
    }

    tresult PLUGIN_API initialize (FUnknown* context) override
    {
        tresult result = AudioEffect::initialize (context);
        if (result == kResultTrue)
        {
            addAudioOutput (STR16 ("Audio Output"), SpeakerArr::kStereo);
        }
        return result;
    }

    tresult PLUGIN_API process (ProcessData& data) override
    {
        return kResultTrue;
    }
};

static FUnknown* createProcessor (void*) { return (IAudioProcessor*)new PeriodicInstrumentsProcessor (); }

extern "C" {

__attribute__((visibility("default"))) __attribute__((used)) IPluginFactory* GetPluginFactory ()
{
    if (!gPluginFactory)
    {
        static PFactoryInfo factoryInfo ("AlZrSe",
            "https://github.com/AlZrSe/periodic-instruments",
            "",
            Vst::kDefaultFactoryFlags);
        gPluginFactory = new CPluginFactory (factoryInfo);

        static PClassInfo processorInfo (kProcessorUID,
            PClassInfo::kManyInstances,
            kVstAudioEffectClass,
            "Periodic Instruments");
        gPluginFactory->registerClass (&processorInfo, createProcessor);
    }
    else
    {
        gPluginFactory->addRef ();
    }

    return gPluginFactory;
}

} // extern "C"
