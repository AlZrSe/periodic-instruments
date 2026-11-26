#include <dlfcn.h>
#include <cstdio>
#include <cstdlib>

#include "pluginterfaces/base/fplatform.h"
#include "public.sdk/source/main/pluginfactory.h"

using namespace Steinberg;

int main() {
    const char* path = "target/release/Periodic Instruments.vst3/Contents/x86_64-linux/Periodic Instruments.so";
    void* lib = dlopen(path, RTLD_NOW);
    if (!lib) { printf("dlopen failed: %s\n", dlerror()); return 1; }
    printf("dlopen OK\n");

    typedef bool (*ModuleEntryFn)(void*);
    ModuleEntryFn me = (ModuleEntryFn)dlsym(lib, "ModuleEntry");
    printf("ModuleEntry ptr: %p\n", (void*)me);
    if (me) { bool r = me(lib); printf("ModuleEntry returned: %d\n", (int)r); }

    typedef IPluginFactory* (*GetFactoryFn)();
    GetFactoryFn gf = (GetFactoryFn)dlsym(lib, "GetPluginFactory");
    printf("GetPluginFactory ptr: %p\n", (void*)gf);
    if (!gf) { printf("dlsym failed: %s\n", dlerror()); return 1; }
    IPluginFactory* factory = gf();
    printf("factory ptr: %p\n", (void*)factory);
    if (!factory) { printf("factory null\n"); return 1; }

    PFactoryInfo info;
    factory->getFactoryInfo(&info);
    printf("vendor: %s\n", info.vendor);
    printf("numClasses: %d\n", (int)factory->countClasses());

    for (int i = 0; i < factory->countClasses(); i++) {
        PClassInfo ci;
        factory->getClassInfo(i, &ci);
        printf("class %d: category='%s' name='%s'\n", i, ci.category, ci.name);
    }

    typedef bool (*ModuleExitFn)();
    ModuleExitFn mx = (ModuleExitFn)dlsym(lib, "ModuleExit");
    if (mx) mx();
    dlclose(lib);
    return 0;
}
