# Simple OpenSim Simulator Monitor

This is a simple monitor for OpenSim Simulators.

The config data can be obtained from your grid owners or managers.  It can also be found by checking under your viewers about info.

At this time, endpoint is only jsonSimStats, This might be expanded to other monitoring endpoints later.  Changing it will result in error.

```json
{
    "Dilatn":"1",
    "SimFPS":"55",
    "PhyFPS":"55",
    "AgntUp":"0",
    "RootAg":"0",
    "ChldAg":"0",
    "NPCAg":"0",
    "Prims":"76",
    "AtvPrm":"0",
    "AtvScr":"14",
    "ScrLPS":"0",
    "ScrEPS":"12",
    "PktsIn":"1",
    "PktOut":"1",
    "PendDl":"0",
    "PendUl":"0",
    "UnackB":"0",
    "TotlFt":"18.18",
    "NetFt":"0",
    "PhysFt":"0.06",
    "OthrFt":"0.01",
    "AgntFt":"0",
    "ImgsFt":"0",
    "FrameDilatn":"1",
    "Logging in Users":"0",
    "GeoPrims":"76",
    "Mesh Objects":"5",
    "Script Engine Thread Count":"0",
    "Util Thread Count":"0",
    "System Thread Count":"84",
    "System Thread Active":"0",
    "ProcMem":"798116",
    "Memory":"416",
    "Uptime":"39.01:28:16.9796380",
    "Version":"OpenSim-NGC 0.9.2.2021.08 Yeti Dev  ",
    "RegionName":"Unknown"
}
```