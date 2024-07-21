#include "pch.h"
#include "PrimaryRadarPlugIn2.h"
#include <Windows.h>
#include <iostream>
#include <string>
#include <stdio.h>
#include <windows.h>
#include "helperlib.h"
#include "EuroScopePlugIn.h"
#include "PrimaryRadarScreen.h"
#include <future>

using namespace std;

#define PLUGIN_NAME      "PrimaryRadar"
#define PLUGIN_VERSION   "1.0"
#define PLUGIN_DEVELOPER "Vicente Rendo"
#define PLUGIN_COPYRIGHT "GPL v3" 

void ErrorExit(const char* msg) {
    std::cerr << msg << " (" << GetLastError() << ")" << std::endl;
    exit(EXIT_FAILURE);
}

static bool startsWith(const char* pre, const char* str) {
	const size_t lenpre = strlen(pre), lenstr = strlen(str);
	return lenstr < lenpre ? false : strncmp(pre, str, lenpre) == 0;
}

inline const char* const BoolToString(bool b) {
	return b ? "true" : "false";
}



CPrimaryRadarScreen::CPrimaryRadarScreen() {}

void CPrimaryRadarScreen::OnAsrContentLoaded(bool Loaded) {

}

void CPrimaryRadarScreen::OnAsrContentToBeSaved(void) {

}

void CPrimaryRadarScreen::OnRefresh(HDC hDC, int Phase) {
	if (Phase != EuroScopePlugIn::REFRESH_PHASE_AFTER_LISTS)
	{
		return;
	}
	EuroScopePlugIn::CRadarTarget rt = GetPlugIn()->RadarTargetSelectFirst();
	while (true) {
		if (!rt.IsValid()) break;
		EuroScopePlugIn::CPosition rtpos = rt.GetPosition().GetPosition();

		double radarPoint[3] = { 40.32195, -7.6134638889, 2050.0 };
		double acPoint[3] = {
			rtpos.m_Latitude,
			rtpos.m_Longitude,
			round(rt.GetPosition().GetPressureAltitude() / 3.281)
		};

		if (rtpos.m_Latitude == 0.0) {
			(string)"";
		}

		auto cs = rt.GetCallsign();
		GetPlugIn()->DisplayUserMessage(PLUGIN_NAME, "", (std::to_string(rtpos.m_Latitude) + " " + rt.GetCallsign() + " " + std::to_string(rtpos.m_Longitude) + " " + std::to_string(round(rt.GetPosition().GetPressureAltitude() / 3.281))).c_str(), true, true, true, false, false);

		bool lineofsight = los(&radarPoint, &acPoint);
		if (lineofsight && !rt.GetPosition().GetTransponderC()) {
			POINT sp1 = ConvertCoordFromPositionToPixel(rtpos);
			RECT area{};
			area.left = sp1.x - 5;
			area.top = sp1.y - 5;
			area.right = sp1.x + 100;
			area.bottom = sp1.y + 5;
			Ellipse(hDC, sp1.x -5, sp1.y - 5, sp1.x + 5, sp1.y + 5);
			string as = std::to_string(rt.GetPosition().GetPressureAltitude());
			as = as + as;
			const char* ac = as.c_str();
			TextOutA(hDC, sp1.x + 10, sp1.y - 6, (LPCSTR)ac, lstrlen((LPCWSTR)ac));
			AddScreenObject(0, "XXX0000", area, false, rt.GetCallsign());
		}
		rt = GetPlugIn()->RadarTargetSelectNext(rt);
	}
}



CPrimaryRadarPlugIn::CPrimaryRadarPlugIn(void)
	: CPlugIn(EuroScopePlugIn::COMPATIBILITY_CODE,
		PLUGIN_NAME,
		PLUGIN_VERSION,
		PLUGIN_DEVELOPER,
		PLUGIN_COPYRIGHT)
{
	loaded = false;
	multithread(&CPrimaryRadarPlugIn::start);
}

EuroScopePlugIn::CRadarScreen* CPrimaryRadarPlugIn::OnRadarScreenCreated(const char* sDisplayName,
	bool NeedRadarContent,
	bool GeoReferenced,
	bool CanBeSaved,
	bool CanBeCreated) {
	if (string(sDisplayName) == "Standard ES radar screen") {
		sendMessage(sDisplayName);
		return new CPrimaryRadarScreen;
	}
}

void CPrimaryRadarPlugIn::start() {
	load();
	loaded = true;
}

void CPrimaryRadarPlugIn::sendMessage(std::string message) {
	DisplayUserMessage(PLUGIN_NAME, "", message.c_str(), true, true, true, false, false);
};

void CPrimaryRadarPlugIn::OnTimer(int Counter) {
}

void CPrimaryRadarPlugIn::multithread(void (CPrimaryRadarPlugIn::* f)()) {
	try {
		thread* mythread = new thread(f, this);
		mythread->detach();
	}
	catch (std::exception e) {
		cout << "Failed to multi-thread function";
	}
}