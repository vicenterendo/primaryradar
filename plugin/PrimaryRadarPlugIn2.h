#pragma once
#include <EuroScopePlugIn.h>
#include <string>
#include <string>
#include <iostream>
#include <string>
#include <thread>
#include <vector>
#include <iostream>
#include <fstream>
#include "StdAfx.h"

class CPrimaryRadarPlugIn :
    public EuroScopePlugIn::CPlugIn
{
    public:
        CPrimaryRadarPlugIn(void);
        void sendMessage(std::string message);
        void CPrimaryRadarPlugIn::OnTimer(int Counter);
        void CPrimaryRadarPlugIn::start();
        void CPrimaryRadarPlugIn::multithread(void (CPrimaryRadarPlugIn::*f)());
        bool CPrimaryRadarPlugIn::OnCompileCommand(const char* sCommandLine);
        EuroScopePlugIn::CRadarScreen* OnRadarScreenCreated(const char* sDisplayName,
            bool NeedRadarContent,
            bool GeoReferenced,
            bool CanBeSaved,
            bool CanBeCreated);
       
        bool loaded;
};

void __declspec (dllexport) EuroScopePlugInInit(EuroScopePlugIn::CPlugIn** ppPlugInInstance);

void __declspec (dllexport) EuroScopePlugInExit(void);