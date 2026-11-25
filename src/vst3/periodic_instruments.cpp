// Periodic Instruments - Minimal VST3 Plugin
// A VST3 instrument plugin featuring 94 elemental voices (Hydrogen through Plutonium)

#include "public.sdk/source/vst/vsteditcontroller.h"
#include "public.sdk/source/vst/vstaudioprocessor.h"
#include "pluginterfaces/vst/ivstparameterchanges.h"
#include "pluginterfaces/vst/ivstaudioprocessor.h"
#include "pluginterfaces/base/ustring.h"
#include "pluginterfaces/base/funknown.h"
#include "base/source/fstring.h"
#include "base/source/fdebug.h"

using namespace Steinberg;
using namespace Steinberg::Vst;

//------------------------------------------------------------------------------------------------------
// Plugin IDs
//------------------------------------------------------------------------------------------------------
// {70657269-6F64-6963-496E-737472756D00}  "PeriodicInstrum" (16 bytes)
static const FUID kPeriodicInstrumentsProcessorUID(0x70657269, 0x6F646963, 0x496E7374, 0x72756D00);
// {70657269-6F64-6963-496E-737472430000}  "PeriodicInstrC" (16 bytes)
static const FUID kPeriodicInstrumentsControllerUID(0x70657269, 0x6F646963, 0x496E7374, 0x72430000);

//------------------------------------------------------------------------------------------------------
// Parameter IDs
//------------------------------------------------------------------------------------------------------
enum ParamID {
    kElementSelector = 0,
    kMass = 1,
    kReactivity = 2,
    kState = 3,
    kRadiance = 4,
    kNumParams = 5
};

//------------------------------------------------------------------------------------------------------
// Audio Processor
//------------------------------------------------------------------------------------------------------
class PeriodicInstrumentsProcessor : public AudioProcessor {
public:
    PeriodicInstrumentsProcessor() : AudioProcessor(), mParams(), mBypass(false) {
        setControllerClass(kPeriodicInstrumentsControllerUID);
    }

    // AudioProcessor
    tresult PLUGIN_API initialize(FUnknown* context) override {
        tresult result = AudioProcessor::initialize(context);
        if (result == kResultTrue) {
            // Add audio inputs/outputs
            addAudioInput(USTRING("Audio Input"), SpeakerArr::kStereo);
            addAudioOutput(USTRING("Audio Output"), SpeakerArr::kStereo);
        }
        return result;
    }

    tresult PLUGIN_API terminate() override {
        return AudioProcessor::terminate();
    }

    tresult PLUGIN_API setActive(TBool state) override {
        return AudioProcessor::setActive(state);
    }

    tresult PLUGIN_API setupProcessing(ProcessSetup& setup) override {
        if (setup.maxSamplesPerBlock > 4096)
            return kResultFalse;
        if (setup.symbolicSampleSize != kSample32)
            return kResultFalse;
        return AudioProcessor::setupProcessing(setup);
    }

    tresult PLUGIN_API canProcessSampleSize(int32 symbolicSampleSize) override {
        return (symbolicSampleSize == kSample32) ? kResultTrue : kResultFalse;
    }

    tresult PLUGIN_API process(ProcessData& data) override {
        // Bypass handling
        if (data.processMode & ProcessMode::kBypass) {
            // Pass-through: copy inputs to outputs
            if (data.inputs && data.outputs) {
                for (int32 i = 0; i < data.numInputs; ++i) {
                    if (data.inputs[i].numChannels == data.outputs[i].numChannels) {
                        for (int32 ch = 0; ch < data.inputs[i].numChannels; ++ch) {
                            if (data.inputs[i].channelBuffers32[ch] && data.outputs[i].channelBuffers32[ch]) {
                                memcpy(data.outputs[i].channelBuffers32[ch],
                                       data.inputs[i].channelBuffers32[ch],
                                       data.numSamples * sizeof(float));
                            }
                        }
                    }
                }
            }
            return kResultTrue;
        }

        // Instrument mode: output silence (foundation build)
        if (data.outputs) {
            for (int32 i = 0; i < data.numOutputs; ++i) {
                for (int32 ch = 0; ch < data.outputs[i].numChannels; ++ch) {
                    if (data.outputs[i].channelBuffers32[ch]) {
                        memset(data.outputs[i].channelBuffers32[ch], 0,
                               data.numSamples * sizeof(float));
                    }
                }
            }
        }
        return kResultTrue;
    }

    tresult PLUGIN_API getTailSize(int32& tailSize) override {
        tailSize = 0;
        return kResultTrue;
    }

    tresult PLUGIN_API setBusArrangements(SpeakerArrangement* inputs, int32 numIns,
                                          SpeakerArrangement* outputs, int32 numOuts) override {
        if (numIns > 0 && inputs[0] != SpeakerArr::kStereo)
            return kResultFalse;
        if (numOuts > 0 && outputs[0] != SpeakerArr::kStereo)
            return kResultFalse;
        return AudioProcessor::setBusArrangements(inputs, numIns, outputs, numOuts);
    }

    tresult PLUGIN_API getBusArrangement(BusDirection dir, int32 index, SpeakerArrangement& arr) override {
        return AudioProcessor::getBusArrangement(dir, index, arr);
    }

    tresult PLUGIN_API setupProcessing(ProcessSetup& setup) override {
        if (setup.maxSamplesPerBlock > 4096)
            return kResultFalse;
        if (setup.symbolicSampleSize != kSample32)
            return kResultFalse;
        return AudioProcessor::setupProcessing(setup);
    }

    tresult PLUGIN_API setActive(TBool state) override {
        return AudioProcessor::setActive(state);
    }

    tresult PLUGIN_API setState(IBStream* state) override {
        return kResultTrue;
    }

    tresult PLUGIN_API getState(IBStream* state) override {
        return kResultTrue;
    }

private:
    ParamValue mParams[kNumParams] = {0, 0.5, 0.5, 0.5, 0.5};
    bool mBypass = false;
};

//------------------------------------------------------------------------------------------------------
// Controller
//------------------------------------------------------------------------------------------------------
class PeriodicInstrumentsController : public EditController {
public:
    PeriodicInstrumentsController() = default;

    // EditController
    tresult PLUGIN_API initialize(FUnknown* context) override {
        tresult result = EditController::initialize(context);
        if (result == kResultTrue) {
            // Add parameters
            parameters.addParameter(STR16("Element Selector"), nullptr, kElementSelector, 0.0,
                                   ParameterInfo::kCanAutomate | ParameterInfo::kIsList,
                                   93); // 94 elements (1-94)
            parameters.addParameter(STR16("Mass"), nullptr, kMass, 0.5,
                                   ParameterInfo::kCanAutomate, 0);
            parameters.addParameter(STR16("Reactivity"), nullptr, kReactivity, 0.5,
                                   ParameterInfo::kCanAutomate, 0);
            parameters.addParameter(STR16("State"), nullptr, kState, 0.5,
                                   ParameterInfo::kCanAutomate, 0);
            parameters.addParameter(STR16("Radiance"), nullptr, kRadiance, 0.5,
                                   ParameterInfo::kCanAutomate, 0);
        }
        return result;
    }

    tresult PLUGIN_API terminate() override {
        return EditController::terminate();
    }

    tresult PLUGIN_API getParameterInfo(int32 paramIndex, ParameterInfo& info) override {
        if (paramIndex >= kNumParams)
            return kResultFalse;

        const char* titles[] = {"Element Selector", "Mass", "Reactivity", "State", "Radiance"};
        const char* shortTitles[] = {"Element", "Mass", "React", "State", "Rad"};
        int32 stepCounts[] = {93, 0, 0, 0, 0};
        int32 flags[] = {
            ParameterInfo::kCanAutomate | ParameterInfo::kIsList,
            ParameterInfo::kCanAutomate,
            ParameterInfo::kCanAutomate,
            ParameterInfo::kCanAutomate,
            ParameterInfo::kCanAutomate
        };

        String str(titles[paramIndex]);
        UString(info.title, 64).assign(str);
        String shortStr(shortTitles[paramIndex]);
        UString(info.shortTitle, 64).assign(shortStr);
        UString(info.units, 64).assign(USTRING(""));
        
        info.id = paramIndex;
        info.stepCount = stepCounts[paramIndex];
        info.defaultNormalizedValue = (paramIndex == 0) ? 0.0 : 0.5;
        info.unitId = 0;
        info.flags = flags[paramIndex];
        info.type = kParameterTypeFloat;
        
        return kResultTrue;
    }

    tresult PLUGIN_API getParamNormalized(ParamID tag) override {
        switch (tag) {
            case kElementSelector: return 0.0;
            case kMass: return 0.5;
            case kReactivity: return 0.5;
            case kState: return 0.5;
            case kRadiance: return 0.5;
            default: return 0.0;
        }
    }

    tresult PLUGIN_API setParamNormalized(ParamID tag, ParamValue value) override {
        return kResultTrue;
    }

    tresult PLUGIN_API getMidiControllerAssignment(int32 busIndex, int16 channel,
                                                   CtrlNumber midiCC, ParamID& tag) override {
        return kNotImplemented;
    }

    IPlugView* PLUGIN_API createView(const char* name) override {
        if (strcmp(name, "editor") == 0) {
            return new PeriodicInstrumentsEditor(this);
        }
        return nullptr;
    }
};

//------------------------------------------------------------------------------------------------------
// Editor
//------------------------------------------------------------------------------------------------------
class PeriodicInstrumentsEditor : public CPluginView {
public:
    PeriodicInstrumentsEditor(EditController* controller) : CPluginView(controller) {
        rect.left = 0;
        rect.top = 0;
        rect.right = 1200;
        rect.bottom = 800;
    }

    tresult PLUGIN_API getSize(ViewRect* rect) override {
        *rect = rect;
        return kResultTrue;
    }

    tresult PLUGIN_API onWheel(float distance) override {
        return kResultTrue;
    }
};

//------------------------------------------------------------------------------------------------------
// Factory
//------------------------------------------------------------------------------------------------------
class PeriodicInstrumentsFactory : public IPluginFactory3 {
public:
    // IPluginFactory
    tresult PLUGIN_API getFactoryInfo(PFactoryInfo* info) override {
        if (!info) return kResultFalse;
        strcpy(info->vendor, "AlZrSe");
        strcpy(info->url, "https://github.com/AlZrSe/periodic-instruments");
        strcpy(info->email, "");
        info->flags = 0;
        return kResultTrue;
    }

    int32 PLUGIN_API countClasses(StrPtr category) override {
        return 2; // Processor + Controller
    }

    tresult PLUGIN_API getClassInfo(int32 index, PClassInfo info) override {
        if (index == 0) {
            // Processor class
            info->cid = kPeriodicInstrumentsProcessorUID;
            info->category = kVstAudioEffectClass;
            strcpy(info->name, "Periodic Instruments");
            strcpy(info->vendor, "AlZrSe");
            strcpy(info->version, "0.1.0");
            info->sdkVersion = VST_VERSION_STRING;
            info->flags = 0;
            strcpy(info->subCategories, "Instrument|Synth");
            return kResultTrue;
        } else if (index == 1) {
            // Controller class
            info->cid = kPeriodicInstrumentsControllerUID;
            info->category = kVstComponentControllerClass;
            strcpy(info->name, "Periodic Instruments Controller");
            strcpy(info->vendor, "AlZrSe");
            strcpy(info->version, "0.1.0");
            info->sdkVersion = VST_VERSION_STRING;
            info->flags = 0;
            return kResultTrue;
        }
        return kResultFalse;
    }

    tresult PLUGIN_API createInstance(const TUID& cid, const TUID& iid, void** obj) override {
        if (cid == kPeriodicInstrumentsProcessorUID) {
            if (iid == IAudioProcessor::iid || iid == IComponent::iid || iid == FUnknown::iid) {
                PeriodicInstrumentsProcessor* instance = new PeriodicInstrumentsProcessor();
                instance->addRef();
                *obj = instance;
                return kResultTrue;
            }
        } else if (cid == kPeriodicInstrumentsControllerUID) {
            if (iid == IEditController::iid || iid == IComponent::iid || iid == FUnknown::iid) {
                PeriodicInstrumentsController* instance = new PeriodicInstrumentsController();
                instance->addRef();
                *obj = instance;
                return kResultTrue;
            }
        }
        *obj = nullptr;
        return kNoInterface;
    }

    // IPluginFactory2
    tresult PLUGIN_API createInstance(const TUID& cid, const TUID& iid, void** obj) override {
        return IPluginFactory3::createInstance(cid, iid, obj);
    }

    // IPluginFactory3
    tresult PLUGIN_API getClassInfo2(int32 index, PClassInfo2 info) override {
        if (index >= countClasses(nullptr))
            return kResultFalse;
        
        PClassInfo info1;
        tresult result = getClassInfo(index, info1);
        if (result != kResultTrue)
            return result;
        
        if (info) {
            info->cid = info1->cid;
            info->category = info1->category;
            strcpy(info->name, info1->name);
            strcpy(info->vendor, info1->vendor);
            strcpy(info->version, info1->version);
            info->sdkVersion = info1->sdkVersion;
            info->flags = info1->flags;
            strcpy(info->subCategories, info1->subCategories);
            info->numAudioInputs = 0;
            info->numAudioOutputs = 2;
            info->numMidiInputs = 1;
            info->numMidiOutputs = 0;
        }
        return kResultTrue;
    }

    // FUnknown
    uint32 PLUGIN_API addRef() override { return ++mRefCount; }
    uint32 PLUGIN_API release() override {
        uint32 count = --mRefCount;
        if (count == 0) delete this;
        return count;
    }
    tresult PLUGIN_API queryInterface(const TUID& iid, void** obj) override {
        if (iid == IPluginFactory::iid || iid == IPluginFactory2::iid || 
            iid == IPluginFactory3::iid || iid == FUnknown::iid) {
            addRef();
            *obj = this;
            return kResultTrue;
        }
        *obj = nullptr;
        return kNoInterface;
    }

private:
    int32 mRefCount = 1;
};

//------------------------------------------------------------------------------------------------------
// Entry Point
//------------------------------------------------------------------------------------------------------
PeriodicInstrumentsFactory gFactory;

extern "C" {

SMTG_EXPORT_FACTORY(gFactory)

bool InitModule() {
    return true;
}

bool DeinitModule() {
    return true;
}

} // extern "C"