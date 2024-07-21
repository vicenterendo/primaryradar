// PrimaryRadarPlugIn.h : main header file for the PrimaryRadarPlugIn DLL
//

#pragma once

#ifndef __AFXWIN_H__
	#error "include 'pch.h' before including this file for PCH"
#endif

#include "resource.h"		// main symbols


// CPrimaryRadarPlugInApp
// See PrimaryRadarPlugIn.cpp for the implementation of this class
//

class CPrimaryRadarPlugInApp : public CWinApp
{
public:
	CPrimaryRadarPlugInApp();

// Overrides
public:
	virtual BOOL InitInstance();

	DECLARE_MESSAGE_MAP()
};
