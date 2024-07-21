#include "EuroScopePlugIn.h"
#include "PrimaryRadarPlugIn2.h"

class CPrimaryRadarScreen :
    public EuroScopePlugIn::CRadarScreen
{

public:
    CPrimaryRadarScreen::CPrimaryRadarScreen();
    virtual void CPrimaryRadarScreen::OnRefresh(HDC hDC, int Phase);
    virtual void CPrimaryRadarScreen::OnAsrContentLoaded(bool Loaded);
    virtual void CPrimaryRadarScreen::OnAsrContentToBeSaved(void);
    virtual inline void CPrimaryRadarScreen::OnAsrContentToBeClosed(void)
    {
        delete this;
    };
};

#include "pch.h"