PROGRAM EstWhi;

(* PROGRAM DECSRIPTION
*  ===================
*
*
*  PROGRAM INFORMATION
*  ===================
*  Author   : M G Davidson
*  Version  : 1.2
*  Date     : 01/08/2002
*  Language : TP4W v1.5 (Borland Turbo Pascal for Windows v1.5)
*                        
*)

USES Win31, WinTypes, WinProcs, WObjects, StdDlgs, WinDos, Strings, Ctl3d;


CONST
  CM_GAMEDEAL  = 100;    (* GAME|DEAL menu option *)
  CM_GAMESCOR  = 101;    (* GAME|SCORES menu option *)
  CM_GAMEOPTN  = 102;    (* GAME|OPTIONS menu option *)
  CM_GAMERAND  = 103;    (* GAME|RANDOM menu option *)
  CM_GAMEEXIT  = 104;    (* GAME|EXIT menu option *)
  CM_HELPCTNT  = 900;    (* HELP|CONTENTS option *)
  CM_HELPHELP  = 901;    (* HELP|HELP ON HELP menu option *)
  CM_HELPABOT  = 999;    (* HELP|ABOUT menu option *)
  ID_CALLZER   = 400;    (* CallWin call zero control ID *)
  ID_CALLONE   = 401;    (* CallWin call one control ID *)
  ID_CALLTWO   = 402;    (* CallWin call two control ID *)
  ID_CALLTHR   = 403;    (* CallWin call three control ID *)
  ID_CALLFOU   = 404;    (* CallWin call four control ID *)
  ID_CALLFIV   = 405;    (* CallWin call five control ID *)
  ID_CALLSIX   = 406;    (* CallWin call six control ID *)
  ID_CALLSEV   = 407;    (* CallWin call seven control ID *)
  ID_CALLEIG   = 408;    (* CallWin call eight control ID *)
  ID_CALLNIN   = 409;    (* CallWin call nine control ID *)
  ID_CALLTEN   = 410;    (* CallWin call ten control ID *)
  ID_CALLELE   = 411;    (* CallWin call eleven control ID *)
  ID_CALLTWE   = 412;    (* CallWin call twelve control ID *)
  ID_CALLTHT   = 413;    (* CallWin call thirteen control ID *)
  ID_CALLFOT   = 414;    (* CallWin call fourteen control ID *)
  ID_CALLFIF   = 415;    (* CallWin call fifteen control ID *)
  ID_NUMPLAY   = 440;    (* OPTIONS no of players scroll control *)
  ID_NUMCARD   = 441;    (* OPTIONS max no of cards scroll control *)
  ID_NUMPLAYS  = 442;    (* OPTIONS no of players static control *)
  ID_NUMCARDS  = 443;    (* OPTIONS max no cards static control *)
  ID_DIALOGRAD = 450;    (* OPTIONS radio dialog box button *)
  ID_MOUSERAD  = 451;    (* OPTIONS radio mouse click button *)
  ID_VANILLA   = 455;    (* OPTIONS vanilla scoring mode *)
  ID_SQUARED   = 456;    (* OPTIONS squared scoring mode *)
  ID_CHEATCARD = 460;    (* OPTIONS cards cheat option *)
  ID_CONFIRMEX = 461;    (* OPTIONS confirm exit option *)
  ID_SAVEEXIT  = 462;    (* OPTIONS save on exit *)
  ID_ABOTTITLE = 550;    (* ABOUT program title box *)
  ID_COPYRIGHT = 551;    (* ABOUT copyright static text control *)
  ID_REGNAME   = 552;    (* ABOUT licensed to name box *)
  ID_REGCOMP   = 553;    (* ABOUT licensed to company box *)
  ID_VERSIONNO = 554;    (* ABOUT version number *)                       
  ID_RELSEDATE = 555;    (* ABOUT release date *)
  ID_ADDRESS   = 556;    (* ABOUT address *)
  ID_SCRNAME1  = 601;    (* SCORE name 1 ID *)
  ID_SCRNAME2  = 602;    (* SCORE name 2 ID *)
  ID_SCRNAME3  = 603;    (* SCORE name 3 ID *)
  ID_SCRNAME4  = 604;    (* SCORE name 4 ID *)
  ID_SCRNAME5  = 605;    (* SCORE name 5 ID *)
  ID_SCRNAME6  = 606;    (* SCORE name 6 ID *)
  ID_SCRNAME7  = 607;    (* SCORE name 7 ID *)
  ID_SCRNAME8  = 608;    (* SCORE name 8 ID *)
  ID_SCRNAME9  = 609;    (* SCORE name 9 ID *)
  ID_SCRNAME10 = 610;    (* SCORE name 10 ID *)
  ID_SCRVALU1  = 611;    (* SCORE value 1 ID *)
  ID_SCRVALU2  = 612;    (* SCORE value 2 ID *)
  ID_SCRVALU3  = 613;    (* SCORE value 3 ID *)
  ID_SCRVALU4  = 614;    (* SCORE value 4 ID *)
  ID_SCRVALU5  = 615;    (* SCORE value 5 ID *)
  ID_SCRVALU6  = 616;    (* SCORE value 6 ID *)
  ID_SCRVALU7  = 617;    (* SCORE value 7 ID *)
  ID_SCRVALU8  = 618;    (* SCORE value 8 ID *)
  ID_SCRVALU9  = 619;    (* SCORE value 9 ID *)
  ID_SCRVALU10 = 620;    (* SCORE value 10 ID *)
  ID_NAMEINPUT = 650;    (* NAME input in scores box *)
  ID_RNDMULTSC = 660;    (* RANDOM THINGS multiplier scrollbar *)
  ID_RNDNUMBSC = 661;    (* RANDOM THINGS number of scrollbar *)
  ID_RNDTIMESC = 662;    (* RANDOM THINGS time interval scrollbar *)
  ID_RNDMULTST = 663;    (* RANDOM THINGS mulitplier static text *)
  ID_RNDNUMBST = 664;    (* RANDOM THINGS number of static text *)
  ID_RNDTIMEST = 665;    (* RANDOM THINGS time interval static text *)
  ID_RNDEXISCK = 666;    (* RANDOM THINGS exist checkbox *)
  ID_RNDICONCK = 667;    (* RANDOM THINGS icon twirl on *)
  ID_HELP      = 998;    (* GENERAL help button *)
  ID_DEALBUT   = 1100;   (* MAIN WINDOW Deal button control ID *)
  ID_EXITBUT   = 1101;   (* MAIN WINDOW Exit button control ID *)
  ID_RNDTIMER  = 2000;   (* Random things timer ID *)
  ID_ICNTIMER  = 2001;   (* Icon timer ID *)

  (* Other constants *)
  CardWidth    = 71;     (* Playing card width *)
  CardHeight   = 96;     (* Playing card height *)
  SmallWidth   = 31;     (* Small picture width *)                                          
  SmallHeight  = 31;     (* Small picture height *)
  MinWidth     = 20;     (* Miniumum window width *)
  MaxPlayNo    = 6;      (* Maximum number of players *)
  MaxCardNo    = 15;     (* Maximum number of cards per player ever *)
  HumanPlayNo  = 1;      (* Human player number *)
  SmallCardHeight = 55;  (* Small card height *)
  SmallCardWidth  = 41;  (* Small card width *)
  SmallMinWidth   = 25;  (* Small card minimum width *)
  TextCardVals1: ARRAY [1..13] OF CHAR = '         1   ';
  TextCardVals2: ARRAY [1..13] OF CHAR = 'A234567890JQK';
  TextCardSuits: ARRAY [1..4] OF CHAR = 'CDHS';

  (* Note that while MaxCardNo is the maximum allowable number of cards per
     player "ever", MaxCards is the maximum number of cards for this hand *)

  (* Ini and help file names *)
  INIFILENAME  = 'ESTWHI.INI';        (* Initialisation file name *)
  HELPFILENAME = 'ESTWHI.HLP';        (* Help file name *)

TYPE
  (* Position record type *)
  TPosition = RECORD
    Left,
    Right,
    Top,
    Bottom: INTEGER;
  END;

  NextCardStyle = (Dialog, Mouse);
  ScoreStyle = (Vanilla, Squared);


  (* Main application prototypes *)
  TCardApp = OBJECT(TApplication)
    PROCEDURE InitMainWindow; virtual;
  END;

  (* Status bar window prototypes *)
  PStatbar = ^TStatbar;
  TStatbar = OBJECT(TWindow)
    WindowText: ARRAY [0..100] OF CHAR;        (* The main text string *)
    HStatbarFont: HFONT;                       (* Handle to vertical axes font *)
    StatbarFont: TLOGFONT;                     (* Font record for the status bar font *)

    CONSTRUCTOR Init(AParent: PWindowsObject; XPos, YPos, Width, Height: INTEGER);
    DESTRUCTOR  Done; virtual;
    PROCEDURE   GetWindowClass(VAR AWndClass: TWndClass);
                virtual;
    FUNCTION    GetClassName: PChar; virtual;
    PROCEDURE   SetupWindow; virtual;
    PROCEDURE   Paint(PaintDC: HDC; VAR PaintInfo: TPaintStruct); virtual;
    PROCEDURE   SetText(TheDC: HDC; TextString: PChar); virtual;
    PROCEDURE   GetText(TextString: PChar); virtual;
  END;

  (* Probability window *)
  PProbWin = ^TProbWin;
  TProbWin = OBJECT(TWindow)

    CONSTRUCTOR Init(AParent: PWindowsObject; AName: PChar);
    DESTRUCTOR  Done; virtual;
    PROCEDURE   WMVScroll(var Msg: TMessage);
                virtual WM_First + WM_VScroll;
    PROCEDURE   Paint(PaintDC: HDC; var PaintInfo: TPaintStruct);
                virtual;
    PROCEDURE   UpdateScreen(TheDC: HDC);
    FUNCTION    CanClose: BOOLEAN; virtual;
  END;

  (* Name window prototype *)
  PNameBox = ^TNameBox;
  TNameBox = OBJECT(TDialog)
    NameInput: PEdit;

    CONSTRUCTOR Init(AParent: PWindowsObject; AName: PChar);
    PROCEDURE   OK(VAR Msg: TMessage);
                virtual ID_FIRST + ID_OK;
  END;


  (* Scores window prototype *)
  PScoreBox = ^TScoreBox;
  TScoreBox = OBJECT(TDialog)
    Score1, Score2, Score3, Score4, Score5,
    Score6, Score7, Score8, Score9, Score10: PStatic;
    Value1, Value2, Value3, Value4, Value5,
    Value6, Value7, Value8, Value9, Value10: PStatic;

    CONSTRUCTOR Init(AParent: PWindowsObject; AName: PChar);
    PROCEDURE   SetupWindow; virtual;
  END;


  (* About box window prototype - this is needed to avoid tampering using a resource
     editor *)
  PAboutBox = ^TABoutBox;
  TAboutBox = OBJECT(TDialog)
    Text1, Text2, Text3, Text4: PStatic;           (* Static text control handles *)

    CONSTRUCTOR Init(AParent: PWindowsObject; AName: PChar);
    PROCEDURE   SetupWindow; virtual;
  END;


  (* Cheat cards window prototypes *)
  PChetBox = ^TChetBox;
  TChetBox = OBJECT(TWindow)

    CONSTRUCTOR Init(AParent: PWindowsObject; AName: PChar);
    DESTRUCTOR  Done; virtual;
    PROCEDURE   GetWindowClass(VAR AWndClass: TWNDCLASS); virtual;
    PROCEDURE   SetupWindow; virtual;
    PROCEDURE   DrawScreen(TheDC: HDC);
    PROCEDURE   Paint (PaintDC: HDC; VAR PaintInfo: TPaintStruct);
                virtual;
    FUNCTION    CanClose: BOOLEAN; virtual;
    FUNCTION    GetClassName: PChar; virtual;
  END;


  (* Options dialog box prototype *)
  POptions = ^TOptions;
  TOptions = OBJECT(TDialog)
    Rad1, Rad2, Rad3, Rad4: PRadioButton;     (* Handles to 4 radio buttons *)
    Chk1, Chk2, Chk3: PCheckBox;              (* Handle to 1 check box *)
    Scroll1, Scroll2: PScrollbar;             (* Scroll bar handles *)
    Txt1, Txt2: PStatic;                      (* Static text handles *)

    CONSTRUCTOR Init (AParent: PWindowsObject; AName: PChar);
    PROCEDURE   SetupWindow; virtual;
    PROCEDURE   OK (VAR Msg: TMessage);
                  virtual ID_FIRST + ID_OK;
    PROCEDURE   Help (VAR Msg: TMessage);
                virtual ID_FIRST + ID_HELP;
    PROCEDURE   HanNumPlaySc (VAR Msg: TMessage);
                virtual ID_FIRST + ID_NUMPLAY;
    PROCEDURE   HanNumCardSc (VAR Msg: TMessage);
                virtual ID_FIRST + ID_NUMCARD;
  END;


  (* Random things dialog box prototype *)
  PRandom = ^TRandom;
  TRandom = OBJECT(TDialog)
    RndMultSt, RndNumbSt, RndTimeSt: PStatic;
    RndExisCk, RndIconCk: PCheckBox;
    RndMultSc, RndNumbSc, RndTimeSc: PScrollBar;      (* Handles *)

    CONSTRUCTOR Init(AParent: PWindowsObject; AName: PChar);
    PROCEDURE   SetupWindow; virtual;
    PROCEDURE   OK (VAR Msg: TMessage);
                virtual ID_FIRST + ID_OK;
    PROCEDURE   Help (VAR Msg: TMessage);
                virtual ID_FIRST + ID_HELP;
    PROCEDURE   HanRndMultSc (VAR Msg: TMessage);
                virtual ID_FIRST + ID_RNDMULTSC;
    PROCEDURE   HanRndNumbSc (VAR Msg: TMessage);
                virtual ID_FIRST + ID_RNDNUMBSC;
    PROCEDURE   HanRndTImeSc (VAR Msg: TMessage);
                virtual ID_FIRST + ID_RNDTIMESC;
  END;


  (* Call dialog box prototypes - this is the window where players register their calls *)
  PCallWin = ^TCallWin;
  TCallWin = OBJECT(TDialog)
    CallBut0, CallBut1, CallBut2, CallBut3, CallBut4,
    CallBut5, CallBut6, CallBut7, CallBut8, CallBut9,
    CallBut10, CallBut11, CallBut12, CallBut13,
    CallBut14, CallBut15: PButton;                     (* Button handles *)

    CONSTRUCTOR Init (AParent: PWindowsObject; AName: PChar);
    PROCEDURE   SetUpWindow; virtual;
    PROCEDURE   CallZer (VAR Msg: TMessage);
                virtual ID_FIRST + ID_CALLZER;
    PROCEDURE   CallOne (VAR Msg: TMessage);
                virtual ID_FIRST + ID_CALLONE;
    PROCEDURE   CallTwo (VAR Msg: TMessage);
                virtual ID_FIRST + ID_CALLTWO;
    PROCEDURE   CallThr (VAR Msg: TMessage);
                virtual ID_FIRST + ID_CALLTHR;
    PROCEDURE   CallFou (VAR Msg: TMessage);
                virtual ID_FIRST + ID_CALLFOU;
    PROCEDURE   CallFiv (VAR Msg: TMessage);
                virtual ID_FIRST + ID_CALLFIV;
    PROCEDURE   CallSix (VAR Msg: TMessage);
                virtual ID_FIRST + ID_CALLSIX;
    PROCEDURE   CallSev (VAR Msg: TMessage);
                virtual ID_FIRST + ID_CALLSEV;
    PROCEDURE   CallEig (VAR Msg: TMessage);
                virtual ID_FIRST + ID_CALLEIG;
    PROCEDURE   CallNin (VAR Msg: TMessage);
                virtual ID_FIRST + ID_CALLNIN;
    PROCEDURE   CallTen (VAR Msg: TMessage);
                virtual ID_FIRST + ID_CALLTEN;
    PROCEDURE   CallEle (VAR Msg: TMessage);
                virtual ID_FIRST + ID_CALLELE;
    PROCEDURE   CallTwe (VAR Msg: TMessage);
                virtual ID_FIRST + ID_CALLTWE;
    PROCEDURE   CallTht (VAR Msg: TMessage);
                virtual ID_FIRST + ID_CALLTHT;
    PROCEDURE   CallFot (VAR Msg: TMessage);
                virtual ID_FIRST + ID_CALLFOT;
    PROCEDURE   CallFif (VAR Msg: TMessage);
                virtual ID_FIRST + ID_CALLFIF;
  END;


  (* Main window procedure/function prototypes *)
  PMainWindow = ^TMainWindow;
  TMainWindow = OBJECT(TWindow)
    Sel: ARRAY [1..4, 1..5] OF INTEGER;    (* Holds hand cards - AutoCall *)
    CardVal: ARRAY [1..13] OF REAL;        (* Holds card values - AutoCallTwo *)   
    TrumpCardVal: ARRAY [1..13] OF REAL;   (* Holds card values - AutoCallTwo *)
    XPos, YPos: ARRAY [1..10] OF INTEGER;  (* Timer points locations *)
    DkGrayPen: HPEN;                       (* Handle to a dark gray pen *)
    LtGrayPen: HPEN;                       (* Handle to a light gray pen *)
    WhitePen:  HPEN;                       (* Handle to a white pen *)
    IconCount: INTEGER;                    (* Icon to be displayed counter *)
    CardValSetting: INTEGER;               (* A flag value - see AutoCallTwo method *)
    Trump: INTEGER;                        (* What is trump suit, 1..4 range *)
    LastWinner: INTEGER;                   (* Who won last trick, 1..NoPlayers range *)
    WaitForMouseClick: BOOLEAN;            (* Internally used in game advance *)
    SaveExit,                              (* Should we save INI info on exit? *)
    ConfirmExit,                           (* Confirm exit *)
    BoxExistProb: BOOLEAN;                 (* Probability window exists *)
    CheatCards: BOOLEAN;                   (* Do we display cheat cards? *)
    CheatX, CheatY: INTEGER;               (* Cheat box X and Y position *)
    CheatCardsPntr: PChetBox;              (* Handle to cheat window *)
    PntrProb: PProbWin;                    (* Handle to probability window *)
    ICON1, ICON2, ICON3: HIcon;            (* Handles to icons *)
    GreenBrush: HBrush;                    (* Background green brush handle *)
    TGreenBrush: TLogBrush;                (* Background green brush style attributes *)
    BlackBrush: HBrush;                    (* Background black brush handle *)
    TBlackBrush: TLogBrush;                (* Background black brush style attributes *)
    LtGrayBrush: HBrush;                   (* Background grey brush handle *)
    ExitBut, DealBut: PButton;             (* Main window playing area button handles *)
    NextCardNotify: NextCardStyle;         (* Method of notifying end of trick *)
    ScoreMode: ScoreStyle;                 (* Method of calculating scores *)
    Statbarhan: PStatbar;                  (* Handle to the status bar *)


    CONSTRUCTOR Init(AParent: PWindowsObject; AName: PChar);
    DESTRUCTOR  Done; virtual;
    PROCEDURE   GetWindowClass(VAR AWndClass: TWndClass); virtual;
    PROCEDURE   SetUpWindow; virtual;
    PROCEDURE   Paint(PaintDC: HDC; VAR PaintInfo: TPaintStruct);
                virtual;
    PROCEDURE   WMEraseBkGnd(VAR Msg: TMessage);
                virtual WM_FIRST + WM_ERASEBKGND;
    PROCEDURE   WMQueryDragIcon(VAR Msg: TMessage);
                virtual WM_FIRST + WM_QUERYDRAGICON;
    PROCEDURE   WMLButtonDown(VAR Msg: TMessage);
                virtual WM_FIRST + WM_LBUTTONDOWN;
    PROCEDURE   WMLButtonUp(VAR Msg: TMessage);
                virtual WM_FIRST + WM_LBUTTONUP;
    PROCEDURE   WMSize(VAR Msg: TMessage);
                virtual WM_FIRST + WM_SIZE;
    PROCEDURE   WMMenuSelect(VAR Msg: TMessage);
                virtual WM_FIRST + WM_MENUSELECT;
    PROCEDURE   GameDeal(VAR Msg: TMessage);
                virtual CM_FIRST + CM_GAMEDEAL;
    PROCEDURE   GameRand(VAR Msg: TMessage);
                virtual CM_FIRST + CM_GAMERAND;
    PROCEDURE   GameOptn(VAR Msg: TMessage);
                virtual CM_FIRST+ CM_GAMEOPTN;
    PROCEDURE   GameScor(VAR Msg: TMessage);
                virtual CM_FIRST + CM_GAMESCOR;
    PROCEDURE   GameExit(VAR Msg: TMessage);
                virtual CM_FIRST + CM_GAMEEXIT;
    PROCEDURE   HelpCtnt(VAR Msg: TMessage);
                virtual CM_FIRST + CM_HELPCTNT;
    PROCEDURE   HelpHelp(VAR Msg: TMessage);
                virtual CM_FIRST + CM_HELPHELP;
    PROCEDURE   HelpAbot(VAR Msg: TMessage);
                virtual CM_FIRST + CM_HELPABOT;
    PROCEDURE   PR_DealBut(VAR Msg: TMessage);
                virtual ID_FIRST + ID_DEALBUT;
    PROCEDURE   PR_ExitBut(VAR Msg: TMessage);
                virtual ID_FIRST + ID_EXITBUT;
    PROCEDURE   WMTimer(VAR Msg: TMessage);
                virtual WM_TIMER;
    PROCEDURE   StartGame;
    PROCEDURE   StartRound;
    PROCEDURE   GameControlOne;
    PROCEDURE   GameControlTwo(Skip: BOOLEAN);
    PROCEDURE   PlayLead(WhichPlayer: INTEGER);
    PROCEDURE   PlayCard(WhichPlayer: INTEGER);
    PROCEDURE   ObtainLegal(GivenPlayer: INTEGER);
    PROCEDURE   CalculateScore;
    PROCEDURE   DecideWinner;
    PROCEDURE   AutoCall(PlayNo, Place: INTEGER);
    PROCEDURE   AutoCallTwo(PlayNo, Place: INTEGER);
    PROCEDURE   CallValueOne;
    PROCEDURE   CallValueTwo;
    PROCEDURE   CallUa(Suit, CardNo, Play: INTEGER; Add: REAL);
    PROCEDURE   Deal;
    PROCEDURE   Shuffle;
    PROCEDURE   SortCards;
    PROCEDURE   DrawCards(DC: HDC; Redraw: BOOLEAN);
    PROCEDURE   DrawPlayedCards(DC: HDC; Redraw: BOOLEAN; Player: INTEGER);
    PROCEDURE   DrawBMP (DC: HDC; X, Y, BitMap: HBitmap; INV: BOOLEAN);
    PROCEDURE   DrawSmallBMP(DC: HDC; X, Y: INTEGER; BitMap: HBitmap; Style: INTEGER);
    PROCEDURE   DrawControls(DC: HDC);
    PROCEDURE   DrawInformation(DC: HDC; Winner: INTEGER);
    PROCEDURE   DrawBox(TheDC: HDC; X1, Y1, X2, Y2: INTEGER);
    PROCEDURE   SetPosInd(DC: HDC);
    FUNCTION    CanClose: BOOLEAN; virtual;
    FUNCTION    DetermineSuit(GivenVal: INTEGER): INTEGER;
    FUNCTION    GetClassName: PChar; virtual;
  END;


(* Global variables *)
VAR
   CardPic: ARRAY [1..52] OF HBitmap;
   BackPic: ARRAY [1..12] OF HBitmap;
   HertPic, DiamPic, ClubPic, SpdePic, MD_Logo, IC_Logo: HBitmap;
   RoundNo, BackNo, NoCards, LButDown,
   StartPlayer, PlayerToPlay, LastPlayer,
   NoPlayers, PreviousPlayer, RoundStartPlayer,
   StackPntr, MaxCards, NumberofRounds, 
   ProbBoxX, ProbBoxY, ProbBoxW, ProbBoxH: INTEGER;
   Multiplier: INTEGER;                   (* Random walk multiplier *)
   NoThings: INTEGER;                     (* Number of random things *)
   TimeRnd: WORD;                         (* Time interval for random things *)
   TimeIco: INTEGER;                      (* Time interval for icon twirl *)
   ExistRnd: BOOLEAN;                     (* Random things exist *)
   ExistIcn: BOOLEAN;                     (* Icon twirl is on *)
   Cards: ARRAY [1..52] OF INTEGER;
   CardPos: ARRAY [1..MaxCardNo] OF TPosition;
   Score: ARRAY [1..MaxPlayNo] OF INTEGER;
   PlayCall: ARRAY [1..MaxPlayNo] OF INTEGER;
   Tricks: ARRAY [1..MaxPlayNo] OF INTEGER;
   CardPlayed: ARRAY [1..MaxPlayNo] OF INTEGER;
   HandCards, LegalHandCards: ARRAY [1..MaxPlayNo, 1..MaxCardNo] OF INTEGER;
   BestScores: ARRAY [1..10] OF INTEGER;
   BestNames: ARRAY [1..10] OF ARRAY [0..20] OF CHAR;
   TempPlayCall: ARRAY [1..MaxPlayNo] OF REAL;
   NameInputText: ARRAY [0..20] OF CHAR;
   GameInProgress: BOOLEAN;

   Probability: ARRAY [1..MaxPlayNo, 1..52] OF SHORTINT;


(************************************************************************)
(* Methods of TCardApp follow...*)


PROCEDURE TCardApp.InitMainWindow;
BEGIN
  MainWindow := New (PMainWindow, Init(nil, 'Estimation Whist'));
END;


(**********************************************************************)
(* Methods of TStatbar from here onwards *)


CONSTRUCTOR TStatbar.Init(AParent: PWindowsObject; XPos, YPos, Width, Height: INTEGER);
VAR
  ParentRect: TRect;
BEGIN
  (* Call ancestor method *)
  TWindow.Init(AParent, '');

  (* Turn off the MDI flag that parent set *)
  Attr.Style := WS_BORDER OR WS_CHILD OR WS_VISIBLE;

  (* Set up window attributes *)
  WITH Attr DO
  BEGIN
    X := XPos;
    Y := YPos;
    W := Width;
    H := Height;
  END;

  (* Create a font record for the status bar font *)
  WITH StatbarFont DO
  BEGIN
    LFHeight := 16;
    LFWidth := 0;
    LFEscapement := 0;
    LFOrientation := 0;
    LFWeight := FW_NORMAL;
    LFItalic := 0;
    LFUnderline := 0;
    LFStrikeOut := 0;
    LFCharSet := ANSI_CHARSET;
    LFOutPrecision := OUT_STROKE_PRECIS;
    lfClipPrecision := CLIP_DEFAULT_PRECIS;
    lfQuality := PROOF_QUALITY;
    lfPitchAndFamily := FF_DONTCARE;
    StrCopy (@LFFaceName, 'Arial');
  END;
  HStatbarFont := CreateFontIndirect(StatbarFont);
END;


DESTRUCTOR TStatbar.Done;
BEGIN
  (* Delete objects *)
  DeleteObject(HStatbarFont);

  (* Call ancestor method *)
  TWindow.Done;
END;


PROCEDURE TStatbar.GetWindowClass(VAR AWndClass: TWndClass);
BEGIN
  (* Register the window class *)
  TWindow.GetWindowClass(AWndClass);
  AWndClass.Style := CS_HREDRAW OR CS_VREDRAW;
  AWndClass.HIcon := LoadIcon(HInstance, IDI_APPLICATION);
  AWndClass.HBRBackground := COLOR_BTNFACE + 1;
END;


FUNCTION TStatbar.GetClassName: PChar;
BEGIN
  (* Obtain the window class name *)
  GetClassName := 'STATUS_BAR';
END;


PROCEDURE TStatbar.SetupWindow;
BEGIN
  (* Call ancestor method *)
  TWindow.SetupWindow;

  (* Default values *)
  StrCopy(WindowText, 'Estimation Whist - Version 1.2 - Copyright MG Davidson 1994-2002');
END;
                                                     

PROCEDURE TStatbar.Paint(PaintDC: HDC; VAR PaintInfo: TPaintStruct);
BEGIN
  (* Call ancestor method *)
  TWindow.Paint(PaintDC, PaintInfo);

  (* Draw text *)
  SetText(PaintDC, WindowText);
END;


PROCEDURE TStatbar.SetText(TheDC: HDC; TextString: PChar);
VAR
  MadeDC: BOOLEAN;
  TempX: INTEGER;
BEGIN
  (* Set the status bar text to that specified in TextString *)
  strcopy(WindowText, TextString);

  (* Check to see that we are given a DC *)
  IF (TheDC = 0) THEN
  BEGIN
    TheDC := GetDC(HWindow);
    MadeDC := TRUE;
  END
  ELSE
    MadeDC := FALSE;

  TempX := Attr.W-7;
  IF (TempX < 5) THEN TempX := 5;

  (* Draw the surrounding box *)
  SelectObject(TheDC, PMainWindow(Parent)^.DkGrayPen);
  MoveTo(TheDC, TempX, 2);
  LineTo(TheDC, 5, 2);
  LineTo(TheDC, 5, 19);

  SelectObject(TheDC, PMainWindow(Parent)^.WhitePen);
  LineTo(TheDC, TempX, 19);
  LineTo(TheDC, TempX, 3);

  (* Clear the old text box *)
  SelectObject(TheDC, PMainWindow(Parent)^.LtGrayPen);
  SelectObject(TheDC, PMainWindow(Parent)^.LtGrayBrush);
  Rectangle(TheDC, 6, 3, (TempX-1), 19);

  (* Select status bar font *)
  SelectObject(TheDC, HStatbarFont);

  (* Set up colours *)
  SetBkColor(TheDC, GetSysColor(COLOR_BTNFACE));
  SetTextColor(TheDC, RGB(0, 0, 0));

  (* Set up clipping region *)
  IntersectClipRect(TheDC, 5, 2, TempX, 19);

  (* Draw the text *)
  TextOut(TheDC, 10, 3, WindowText, strlen(WindowText));

  (* Reset clipping region *)
  SelectClipRgn(TheDC, 0);

  (* Release DC *)
  IF (MadeDC) THEN
    ReleaseDC(HWindow, TheDC);
END;


PROCEDURE TStatbar.GetText(TextString: PChar);
BEGIN
  StrCopy(TextString, WindowText);
END;


(************************************************************************)
(* Methods of TMainWindow follow...*)


CONSTRUCTOR TMainWindow.Init (AParent: PWindowsObject; AName: PChar);
VAR
  A: INTEGER;
BEGIN
  (* Call ancestor method *)
  TWindow.Init(AParent, AName);

  (* Window attributes *)
  Attr.W := 600;
  Attr.H := 400;
  Attr.X := GetPrivateProfileInt('Main Window', 'Window X position', 0, INIFILENAME);
  Attr.Y := GetPrivateProfileInt('Main Window', 'Window Y position', 0, INIFILENAME);
  Attr.Menu := LoadMenu(HInstance, 'Menu');
  Attr.Style := Attr.Style AND NOT(WS_MAXIMIZEBOX) AND NOT(WS_SIZEBOX);

  (* Load card bitmaps *)
  FOR A := 1 TO 52 DO
    CardPic[A] := LoadBitmap(HInstance, PChar(A));

  (* Icon loads *)
  ICON1 := LoadIcon(HInstance, 'ICON1');
  ICON2 := LoadIcon(HInstance, 'ICON2');
  ICON3 := LoadIcon(HInstance, 'ICON3');
  IconCount := 1;

  (* Load "little" pics *)
  ClubPic := LoadBitmap(HInstance, 'CLUB');
  SpdePic := LoadBitmap(HInstance, 'SPADE');
  DiamPic := LoadBitmap(HInstance, 'DIAMOND');
  HertPic := LoadBitmap(HInstance, 'HEART');
  MD_Logo := LoadBitmap(HInstance, 'MD_LOGO');
  IC_Logo := LoadBitmap(HInstance, 'ICON_LOGO');

  (* Create pens *)
  DkGrayPen := CreatePen(PS_SOLID, 1, GetSysColor(COLOR_BTNSHADOW));
  LtGrayPen := CreatePen(PS_SOLID, 1, GetSysColor(COLOR_BTNFACE));
  WhitePen  := CreatePen(PS_SOLID, 1, GetSysColor(COLOR_BTNHIGHLIGHT));

  (* Create brushes *)
  GreenBrush  := CreateSolidBrush(RGB(0, 128, 0));
  BlackBrush  := CreateSolidBrush(RGB(0, 0, 0));
  LtGrayBrush := CreateSolidBrush(GetSysColor(COLOR_BTNFACE));

  (* Setup playmode buttons *)
  DealBut := New(PButton, Init(@self, ID_DealBut, 'Deal', 530, 232, 45, 45, FALSE));
  ExitBut := New(PButton, Init(@self, ID_ExitBut, 'Exit', 530, 285, 45, 45, FALSE));
END;


DESTRUCTOR TMainWindow.Done;
VAR
   A: INTEGER;
   TextString, TextStrtwo: ARRAY [0..30] OF CHAR;
BEGIN
  (* Delete card and card back bitmaps *)
  FOR A := 1 TO 52 DO
    DeleteObject(CardPic[A]);
  FOR A := 1 TO 12 DO
    DeleteObject(BackPic[A]);
  DeleteObject(ClubPic);
  DeleteObject(HertPic);
  DeleteObject(SpdePic);
  DeleteObject(DiamPic);
  DeleteObject(MD_Logo);
  DeleteObject(IC_Logo);
  DeleteObject(GreenBrush);
  DeleteObject(BlackBrush);
  DeleteObject(LtGrayBrush);
  DestroyIcon(ICON1);
  DestroyIcon(ICON2);
  DestroyIcon(ICON3);

  (* Delete pens *)
  DeleteObject(DkGrayPen);
  DeleteObject(LtGrayPen);
  DeleteObject(WhitePen);

  (* If "SaveExit" then we must save them *)
  IF (SaveExit) THEN
  BEGIN
    (* Cheat window/box *)
    Str(ORD(CheatCards), TextString);
    WritePrivateProfileString('Cheat Cards', 'Window exists', TextString, INIFILENAME);
    Str(CheatX, TextString);
    WritePrivateProfileString('Cheat Cards', 'X position', TextString, INIFILENAME);
    Str(CheatY, TextString);
    WritePrivateProfileString('Cheat Cards', 'Y position', TextString, INIFILENAME);

    (* Exit confirmation *)
    Str(ORD(ConfirmExit), TextString);
    WritePrivateProfileString('Main Window', 'Confirm exit', TextString, INIFILENAME);

    (* Probability window/box *)
    Str(ORD(BoxExistProb), TextString);
    WritePrivateProfileString('Main Window', 'Probability box exists', TextString, INIFILENAME);

    (* Other main window items *)
    CASE NextCardNotify OF
      Dialog : StrCopy(TextString, '0');
      Mouse  : StrCopy(TextString, '1');
    END;
    WritePrivateProfileString('Main Window', 'Next card notify', TextString,
                              INIFILENAME);

    CASE ScoreMode OF
      Vanilla : StrCopy(TextString, '0');
      Squared : StrCopy(TextString, '1');
    END;
    WritePrivateProfileString('Main Window', 'Score mode', TextString,
                              INIFILENAME);

    Str(Attr.X, TextString);
    WritePrivateProfileString('Main Window', 'Window X position', TextString, INIFILENAME);
    Str(Attr.Y, TextString);
    WritePrivateProfileString('Main Window', 'Window Y position', TextString, INIFILENAME);
    Str(NoPlayers, TextString);
    WritePrivateProfileString('Main Window', 'Number of players', TextString, INIFILENAME);
    Str(MaxCards, TextString);
    WritePrivateProfileString('Main Window', 'Maximum no cards', TextString, INIFILENAME);

    (* Random things *)
    Str(Multiplier, TextString);
    WritePrivateProfileString('Random Things', 'Multiplier', TextString, INIFILENAME);
    Str(NoThings, TextString);
    WritePrivateProfileString('Random Things', 'Number of', TextString, INIFILENAME);
    Str(TimeRnd, TextString);
    WritePrivateProfileString('Random Things', 'Time interval', TextString, INIFILENAME);

    (* Random things exist *)
    Str(ORD(ExistRnd), TextString);
    WritePrivateProfileString('Random Things', 'They exist', TextString, INIFILENAME);

    (* Icon twirl *)
    Str(TimeIco, TextString);
    WritePrivateProfileString('Icon Twirl', 'Time interval', TextString, INIFILENAME);

    (* Icon twirl is on *)
    Str(ORD(ExistIcn), TextString);
    WritePrivateProfileString('Icon Twirl', 'Is on', TextString, INIFILENAME);

    (* Probability window *)
    Str(ProbBoxX, TextString);
    WritePrivateProfileString('Probability Window', 'X Position', TextString,
                              INIFILENAME);
    Str(ProbBoxY, TextString);
    WritePrivateProfileString('Probability Window', 'Y Position', TextString,
                              INIFILENAME);
    Str(ProbBoxW, TextString);
    WritePrivateProfileString('Probability Window', 'Width', TextString,
                              INIFILENAME);
    Str(ProbBoxH, TextString);
    WritePrivateProfileString('Probability Window', 'Height', TextString,
                              INIFILENAME);


    (* Yes, we saved on exit *)
    WritePrivateProfileString('Main Window', 'Save on exit', '1', INIFILENAME);
  END
  ELSE
  BEGIN
    (* No, we didn't save on exit *)
    WritePrivateProfileString('Main Window', 'Save on exit', '0', INIFILENAME);
  END;


  (* But we always save the high scores *)
  FOR A := 1 TO 10 DO
  BEGIN
    StrCopy(TextString, 'Value');
    Str(A, TextStrtwo);
    StrCat(TextString, TextStrtwo);
    Str(BestScores[A], TextStrtwo);
    WritePrivateProfileString('Scores', TextString, TextStrtwo,
                              INIFILENAME);
    StrCopy(TextString, 'Name');
    Str(A, TextStrtwo);
    StrCat(TextString, TextStrtwo);
    WritePrivateProfileString('Scores', TextString, BestNames[A],
                              INIFILENAME);
  END;

  (* Close help window *)
  WinHelp(HWindow, HELPFILENAME, HELP_QUIT, 0);

  (* Call ancestor method *)
  TWindow.Done;
END;


PROCEDURE TMainWindow.GetWindowClass(VAR AWndClass: TWndClass);
BEGIN
  (* Window particulars *)
  TWindow.GetWindowClass (AWndClass);
  AWndClass.hIcon := 0;
  AWndClass.hbrBackGround := 0;
END;


PROCEDURE TMainWindow.SetUpWindow;
VAR
  A, B: INTEGER;
  TextString, TextStrtwo: ARRAY [0..40] OF CHAR;
  WinRect: TRECT;
BEGIN
  (* Call ancestor method *)
  TWindow.SetUpWindow;

  (* Default values for various odds and ends *)
  Trump := 1;
  NoPlayers := 4;
  MaxCards := 52 DIV NoPlayers;
  NoCards := 0;
  GameInProgress := FALSE;
  LastWinner := 0;
  WaitForMouseClick := FALSE;
  CardValSetting := 0;

  (* Initialise arrays *)
  FOR A := 1 TO MaxPlayNo DO
  BEGIN
    FOR B := 1 TO MaxCardNo DO
    BEGIN
      HandCards[A, B] := 0;
      LegalHandCards[A, B] := 0;
    END;
  END;

  FOR A := 1 TO MaxPlayNo DO
  BEGIN
    FOR B := 1 TO 52 DO
    BEGIN
      Probability[A, B] := 0;
    END;
  END;

  FOR A := 1 TO 10 DO
  BEGIN
    XPos[A] := (Attr.W DIV 2);
    YPos[A] := (Attr.H DIV 2);
  END;

  (* Get client window size *)
  GetClientRect(HWindow, WinRect);

  (* Create status bar *)
  StatbarHan := New(PStatbar, Init(@self, 0, (WinRect.Bottom - 24),
                        (WinRect.Right - WinRect.Left), 24));
  Application^.MakeWindow(StatbarHan);

  (* Read initialisation file for saved values *)
  SaveExit := BOOL(GetPrivateProfileInt('Main Window', 'Save on exit', 1, INIFILENAME));
  ConfirmExit := BOOL(GetPrivateProfileInt('Main Window', 'Confirm exit', 1, INIFILENAME));

  A := GetPrivateProfileInt('Main Window', 'Next card notify', 1, INIFILENAME);
  CASE A OF
    0 : NextCardNotify := Dialog;
    1 : NextCardNotify := Mouse;
  END;

  A := GetPrivateProfileInt('Main Window', 'Score mode', 0, INIFILENAME);
  CASE A OF
    0 : ScoreMode := Vanilla;
    1 : ScoreMode := Squared;
  END;

  FOR A := 1 TO 10 DO
  BEGIN
    StrCopy(TextString, 'Value');
    Str(A, TextStrtwo);
    StrCat(TextString, TextStrtwo);
    BestScores[A] := GetPrivateProfileInt('Scores', TextString, 10,
                                INIFILENAME);
    StrCopy(TextString, 'Name');
    Str(A, TextStrtwo);
    StrCat(TextString, TextStrtwo);
    GetPrivateProfileString('Scores', TextString, 'Fred Flintstone',
                            BestNames[A], 20, INIFILENAME);
  END;

  NoPlayers := GetPrivateProfileInt('Main Window', 'Number of players', 4, INIFILENAME);
  MaxCards  := GetPrivateProfileInt('Main Window', 'Maximum no cards', 13, INIFILENAME);

  (* Cheat cards *)
  CheatCards := BOOL(GetPrivateProfileInt('Cheat Cards', 'Window exists', 0, INIFILENAME));
  CheatX := GetPrivateProfileInt('Cheat Cards', 'X position', 100, INIFILENAME);
  CheatY := GetPrivateProfileInt('Cheat Cards', 'Y position', 100, INIFILENAME);


  BoxExistProb := BOOL(GetPrivateProfileInt('Main Window', 'Probability box exists', 0, INIFILENAME));
  ProbBoxX := GetPrivateProfileInt('Main Window', 'Probability box X', 600, INIFILENAME);
  ProbBoxY := GetPrivateProfileInt('Main Window', 'Probability box Y', 0, INIFILENAME);


  (* Random things *)
  TimeRnd := GetPrivateProfileInt('Random Things', 'Time interval', 20, INIFILENAME);
  NoThings := GetPrivateProfileInt('Random Things', 'Number of', 6, INIFILENAME);
  Multiplier := GetPrivateProfileInt('Random Things', 'Multiplier', 6, INIFILENAME);

  ExistRnd := BOOL(GetPrivateProfileInt('Random Things', 'They exist', 1, INIFILENAME));

  (* Probability window *)
  ProbBoxX := GetPrivateProfileInt('Probability Window', 'X Position', 600, INIFILENAME);
  ProbBoxY := GetPrivateProfileInt('Probability Window', 'Y Position', 0, INIFILENAME);
  ProbBoxW := GetPrivateProfileInt('Probability Window', 'Width', 100, INIFILENAME);
  ProbBoxH := GetPrivateProfileInt('Probability Window', 'Height', 400, INIFILENAME);

  (* Validate data read in *)
  IF ((TimeRnd > 1000) OR (TimeRnd < 20)) THEN TimeRnd := 200;
  IF ((NoThings > 6) OR (NoThings < 1)) THEN NoThings := 3;
  IF ((Multiplier > 20) OR (Multiplier < 1)) THEN Multiplier := 5;
                                                                        
  (* Icon twirl  - and validation *)
  TimeIco := GetPrivateProfileInt('Icon Twirl', 'Time interval', 1000, INIFILENAME);
  ExistIcn := BOOL(GetPrivateProfileInt('Icon Twirl', 'Is on', 1, INIFILENAME));
  IF (TimeIco <> 1000) THEN TimeIco := 1000;

  (* If cheat box is meant to exist then so it should *)
  IF (CheatCards) THEN  
  BEGIN
    CheatCardsPntr := New(PChetBox, Init(@self, 'Cheat Information - Cards'));
    IF (Application^.MakeWindow(CheatCardsPntr) = nil) THEN
    BEGIN
      MessageBox(HWindow,
                 'Could not create window object!',
                 'Error:',
                 MB_ICONHAND);
      CheatCards := FALSE;
    END;
  END;

  (* Program initialisation *)
  RANDOMIZE;

  (* Get a timer for random things *)
  IF (ExistRnd) THEN  SetTimer(HWindow, ID_RNDTIMER, TimeRnd, NIL);
END;


PROCEDURE TMainWindow.Paint(PaintDC: HDC; VAR PaintInfo: TPaintStruct);
VAR
   A: INTEGER;
   Rectangle: TRect;
BEGIN
  IF (IsIconic(HWindow)) THEN
  BEGIN
    SetBkMode(PaintDC, TRANSPARENT);

    (* Paint the desktop window background *)
    DefWindowProc(HWindow, WM_ICONERASEBKGND, PaintDC, 0);

    (* Draw icon *)
    CASE IconCount OF
      1 : DrawIcon(PaintDC, 0, 0, ICON1);
      2 : DrawIcon(PaintDC, 0, 0, ICON2);
      3 : DrawIcon(PaintDC, 0, 0, ICON3);
    END;
  END
  ELSE
  BEGIN
    (* Paint background *)
    GetClientRect(HWindow, Rectangle);
    FillRect(PaintDC, Rectangle, GreenBrush);

    (* Draw logo bitmap *)
    IF (NOT(GameInProgress)) THEN DrawSmallBMP(PaintDC, 285, 80, IC_Logo, 1);

    (* Draw cards *)
    DrawCards(PaintDC, TRUE);
    DrawPlayedCards(PaintDC, TRUE, 0);

    (* Draw controls *)
    DrawControls(PaintDC);
    DrawInformation(PaintDC, 0);
  END;
END;


PROCEDURE TMainWindow.WMEraseBkGnd(VAR Msg: TMessage);
BEGIN
  IF (IsIconic(HWindow)) THEN  Msg.Result := WORD(TRUE);
END;


PROCEDURE TMainWindow.WMQueryDragIcon(VAR Msg: TMessage);
BEGIN
  (* Return a handle to the icon to use as a default *)
  Msg.Result := ICON1;
END;


PROCEDURE TMainWindow.WMLButtonDown(VAR Msg: TMessage);
VAR
   MseXPos, MseYPos, A: INTEGER;
BEGIN
  (* Get mouse positions *)
  MseXPos := LOWORD(Msg.lParam);
  MseYPos := HIWORD(Msg.lParam);
  LButDown := 0;

  (* Work out which card has been selected - if any *)
  IF (PlayerToPlay = 1) THEN
  BEGIN
    FOR A := 1 TO MaxCards DO
    BEGIN
      WITH CardPos[A] DO
      BEGIN
        IF ((MseXPos > Left) AND (MseXPos < Right)) THEN
        BEGIN
          IF ((MseYPos > Top) AND (MseYPos < Bottom)) THEN
            LButDown := A;
        END;
      END;
    END;
  END;
END;


PROCEDURE TMainWindow.WMLButtonUp(VAR Msg: TMessage);
VAR
   MseXPos, MseYPos, A: INTEGER;
BEGIN
  (* Check to see if waiting for a mouse click to advance game *)
  IF (WaitForMouseClick) THEN
  BEGIN
    WaitForMouseClick := FALSE;
    GameControlTwo(TRUE);
    EXIT;
  END;

  (* Get mouse position and store *)
  MseXPos := LOWORD(Msg.lParam);
  MseYPos := HIWORD(Msg.lParam);
  IF (LButDown = 0) THEN EXIT;

  (* Check that mouse is still over the same card *)
  WITH CardPos[LButDown] DO
  BEGIN
    IF ((MseXPos > Left) AND (MseXPos < Right)) THEN
    BEGIN
      IF ((MseYPos > Top) AND (MseYPos < Bottom)) THEN
      BEGIN
        (* Check card is valid *)
        IF (LegalHandCards[PlayerToPlay, LButDown] = 1) THEN
        BEGIN
          (* Reset all cards to be valid *)
          FOR A := 1 TO MaxCards DO
            LegalHandCards[PlayerToPlay, A] := 1;

          (* Has selected this card *)
          CardPlayed[PlayerToPlay] := HandCards[PlayerToPlay, LButDown];
          HandCards[PlayerToPlay, LButDown] := 0;
          DrawCards(0, TRUE);
          INC(PlayerToPlay);
          GameControlTwo(FALSE);
        END;
      END;
    END;
  END;

END;


PROCEDURE TMainWindow.WMSize(VAR Msg: TMessage);
BEGIN
  (* If the window is about to be iconised then create a timer for icon twirl *)
  IF ((IsIconic(HWindow)) AND ExistIcn) THEN
    SetTimer(HWindow, ID_ICNTIMER, TimeIco, NIL)
  ELSE
    KillTimer(HWindow, ID_ICNTIMER);

  (* Call ancestor method *)
  TWindow.WMSize(Msg);
END;


PROCEDURE TMainWindow.WMMenuSelect(VAR Msg: TMessage);
VAR
  hMenu1, hMenu2: HMENU;
  OldText: ARRAY [0..100] OF CHAR;
BEGIN
  (* Handle menu selections by describing menu item's function in the status bar
     - if it exists *)
  hMenu1 := GetSubMenu(GetMenu(HWindow), 0);
  hMenu2 := GetSubMenu(GetMenu(HWindow), 1);

  StatbarHan^.GetText(OldText);

  CASE (Msg.WParam) OF
    CM_GAMEDEAL       : StatbarHan^.SetText(0, 'Deal a new game');
    CM_GAMESCOR       : StatbarHan^.SetText(0, 'Display the top ten high scores');
    CM_GAMEOPTN       : StatbarHan^.SetText(0, 'Configure game options');
    CM_GAMERAND       : StatbarHan^.SetText(0, 'Configure random things');
    CM_GAMEEXIT       : StatbarHan^.SetText(0, 'Exit from Estimation Whist');
    CM_HELPCTNT       : StatbarHan^.SetText(0, 'Help contents for Estimation Whist');
    CM_HELPHELP       : StatbarHan^.SetText(0, 'Help on using Windows help');
    CM_HELPABOT       : StatbarHan^.SetText(0, 'About Estimation Whist');
  ELSE
    IF (IsIconic(HWindow)) THEN
      StatbarHan^.SetText(0, OldText)
    ELSE
      StatbarHan^.SetText(0, '');
  END;

  (* Check for main menu selections *)
  IF ((Msg.WParam = hMenu1) AND (hMenu1 <> 0)) THEN StatbarHan^.SetText(0, 'Game menu');
  IF ((Msg.WParam = hMenu2) AND (hMenu2 <> 0)) THEN StatbarHan^.SetText(0, 'Help menu');
END;


PROCEDURE TMainWindow.GameDeal(VAR Msg: TMessage);
BEGIN
  (* Start game off *)
  StartGame;
  StartRound;
  GameControlOne;
END;


PROCEDURE TMainWindow.GameRand(VAR Msg: TMessage);
VAR
  TempInt, TempRnd: INTEGER;
  TempBool: BOOLEAN;
BEGIN
  TempInt := TimeRnd;
  TempRnd := NoThings; 
  TempBool := ExistRnd;

  (* Bring up random things dialog box *)
  StatbarHan^.SetText(0, 'Use this dialog to configure the random things');
  Application^.ExecDialog(New(PRandom, Init(@self, 'Random')));
  StatbarHan^.SetText(0, '');

  IF (TimeRnd <> TempInt) THEN
  BEGIN
    KillTimer(HWindow, ID_RNDTIMER);
    SetTimer(HWindow, ID_RNDTIMER, TimeRnd, NIL);
  END;

  IF (TempBool <> ExistRnd) THEN
  BEGIN
    (* ExistRnd has changed *)
    IF (ExistRnd AND NOT(GameInProgress)) THEN
      SetTimer(HWindow, ID_RNDTIMER, TimeRnd, NIL)
    ELSE
      KillTimer(HWindow, ID_RNDTIMER);
  END;

  IF ((NoThings <> TempRnd) OR (TempBool <> ExistRnd)) THEN
    InvalidateRect(HWindow, NIL, TRUE);
END;


PROCEDURE TMainWindow.GameOptn(VAR Msg: TMessage);
VAR
   Temp, Temp2: BOOLEAN;
BEGIN
  (* Invoke game options dialog *)
  Temp := GameInProgress;
  Temp2 := CheatCards;
  StatbarHan^.SetText(0, 'Use this dialog to configure the game options');
  Application^.ExecDialog(New(POptions, Init(@self, 'Options')));
  IF (Temp2 <> CheatCards) THEN
  BEGIN
    IF (CheatCards) THEN
    BEGIN
      (* Open a new window *)
      CheatCardsPntr := New(PChetBox, Init(@self, 'Cheat Information - Cards'));
      IF (Application^.MakeWindow(CheatCardsPntr) = NIL) THEN
      BEGIN
        MessageBox(HWindow, 'Could not create window object!',
                   'Error:', MB_ICONHAND);
        CheatCards := FALSE;
      END
    END
    ELSE
    BEGIN
      (* Close the window *)
      CheatCardsPntr^.CloseWindow;
      CheatCards := FALSE;
    END;
  END;

  StatbarHan^.SetText(0, '');
  IF ((Temp) AND NOT(GameInProgress)) THEN GameDeal(Msg);
END;


PROCEDURE TMainWindow.GameScor(VAR Msg: TMessage);
BEGIN
  (* Display scores window *)
  StatbarHan^.SetText(0, 'The top ten high scores');
  Application^.ExecDialog(New(PScoreBox, Init(@self, 'Scores')));
  StatbarHan^.SetText(0, '');
END;


PROCEDURE TMainWindow.GameExit(VAR Msg: TMessage);
BEGIN
  (* Exit game by calling destructor *)
  CloseWindow;
END;                  


PROCEDURE TMainWindow.HelpCtnt(VAR Msg: TMessage);
BEGIN
  (* Calls help files help contents *)
  WinHelp(HWindow, HELPFILENAME, HELP_INDEX, 0);
END;


PROCEDURE TMainWindow.HelpHelp(VAR Msg: TMessage);
BEGIN
  (* Calls Windows help on help functions *)
  WinHelp(HWindow, HELPFILENAME, HELP_HELPONHELP, 0);
END;


PROCEDURE TMainWindow.HelpAbot(VAR Msg: TMessage);
BEGIN
  (* Bring up about box *)
  StatbarHan^.SetText(0, 'Estimation Whist - Version 1.2 - Copyright MG Davidson 1994-2002');
  Application^.ExecDialog(New(PAboutBox, Init(@self, 'AboutBox')));
END;


PROCEDURE TMainWindow.PR_DealBut(VAR Msg: TMessage);
BEGIN
  (* Call deal routine *)
  GameDeal (Msg);
END;


PROCEDURE TMainWindow.PR_ExitBut(VAR Msg: TMessage);
BEGIN
  (* Call destructor *)
  CloseWindow;
END;


PROCEDURE TMainWindow.WMTimer(VAR Msg: TMessage);
VAR
   DC: HDC;
   A: INTEGER;
   Rectangle: TRect;
   TheBitmap: HBitmap;
BEGIN


  IF (Msg.wParam = ID_ICNTIMER) THEN
  BEGIN
    IF (IsIconic(HWindow)) THEN
    BEGIN
      (* Get DC *)
      DC := GetDC(HWindow);
      SetBkMode(DC, Transparent);

      (* Draw icon *)
      CASE IconCount OF
        1 : DrawIcon(DC, 0, 0, ICON1);
        2 : DrawIcon(DC, 0, 0, ICON2);
        3 : DrawIcon(DC, 0, 0, ICON3);
      END; 

      INC(IconCount);
      IF (IconCount > 3) THEN IconCount := 1;

      (* Release DC *)
      ReleaseDC(HWindow, DC);
    END;
  END;


  IF (Msg.wParam = ID_RNDTIMER) THEN
  BEGIN
    IF (NOT(IsIconic(HWindow))) THEN
    BEGIN
      (* Get DC *)
      DC := GetDC(HWindow);

      FOR A := 1 TO NoThings DO
      BEGIN
        (* Clear the old structures from the screen *)
        WITH Rectangle DO
        BEGIN
          Left := XPos[A];
          Right := XPos[A] + 31;
          Top := YPos[A];
          Bottom := YPos[A] + 31;                                                    
        END;
        FillRect(DC, Rectangle, GreenBrush);

        XPos[A] := XPos[A] + Multiplier*(INTEGER(RANDOM(3)) - 1);
        YPos[A] := YPos[A] + Multiplier*(INTEGER(RANDOM(3)) - 1);

        (* This prevents the status bar being eaten up *)
        IF (XPos[A] < 0) THEN XPos[A] := XPos[A] + Multiplier;
        IF (XPos[A] > (Attr.W - 31)) THEN XPos[A] := XPos[A] - Multiplier;
        IF (YPos[A] < 0) THEN YPos[A] := YPos[A] + Multiplier;
        IF (YPos[A] > (Attr.H - 94)) THEN YPos[A] := YPos[A] - Multiplier;

        (* This prevents the main logo square being "eaten" up *)
        IF (((XPos[A] < 316) AND (XPos[A] > 254)) AND
            ((YPos[A] < 111) AND (YPos[A] > 49))) THEN
        BEGIN
          IF (XPos[A] < 285) THEN XPos[A] := 254 ELSE XPos[A] := 316;
          IF (YPos[A] < 80) THEN YPos[A] := 49 ELSE YPos[A] := 111;
        END;

        (* This prevents the buttons being eaten up *)
        IF ((YPos[A] > 202) AND (XPos[A] > 500)) THEN
        BEGIN
          XPos[A] := XPos[A] - Multiplier;
          YPos[A] := YPos[A] - Multiplier;
        END;
      END;

      (* Note to my myself - there is a reason for having two loops which are
         essentially the same - it is to do with drawing the things on the screen.
         If we use just one loop there is a danger that a newly positioned 'thing' will
         be partly rubbed out by the green rectangle drawn to clear an old 'thing'. Thus
         the seperate loop is required. Thanks. *)
      FOR A := 1 TO NoThings DO
      BEGIN
        (* Now select the bitmap... *)
        CASE A OF
          1 : TheBitmap := ClubPic;
          2 : TheBitmap := DiamPic;
          3 : TheBitmap := HertPic;
          4 : TheBitmap := SpdePic;
          5 : TheBitmap := MD_Logo;
          6 : TheBitmap := IC_Logo;
        END;

        (* ...and draw it *)
        DrawSmallBMP(DC, XPos[A], YPos[A], TheBitmap, 1);
      END;

      (* Release DC *)
      ReleaseDC(HWindow, DC);
    END;
  END;
END;


PROCEDURE TMainWindow.StartGame;
VAR
   A: INTEGER;
BEGIN
  (* Get rid of timer for random things if they exist *)
  IF (ExistRnd) THEN  KillTimer(HWindow, ID_RndTimer);

  (* Reset scores *)
  FOR A := 1 TO MaxPlayNo DO
    Score[A] := 0;

  (* Initialise variables *)
  RoundNo := 0;
  NumberofRounds := (2*MaxCards) - 1; 
  RoundStartPlayer := 0;
  Trump := 0;

  (* Reset last winner *)
  LastWinner := 0;

  GameInProgress := TRUE;
END;


PROCEDURE TMainWindow.StartRound;
VAR
   A, B: INTEGER;
BEGIN
  (* Call shuffling/dealing routine *)
  Shuffle;

  (* Reset card played array *)
  FOR A := 1 TO NoPlayers DO
    CardPlayed[A] := 0;

  (* Reset call array *)
  FOR A := 1 TO NoPlayers DO
    PlayCall[A] := 0;

  (* Make all cards legal *)
  FOR A := 1 TO NoPlayers DO
  BEGIN
    FOR B := 1 TO MaxCards DO
      LegalHandCards[A, B] := 1;
  END;

  (* Increment RoundNo *)
  INC(RoundNo);

  (* Increment PlayStart - ie which player starts this round *)
  INC(RoundStartPlayer);
  IF (RoundStartPlayer > NoPlayers) THEN RoundStartPlayer := 1;
  PlayerToPlay := RoundStartPlayer;
  StartPlayer := RoundStartPlayer;
  LastPlayer := RoundStartPlayer - 1;
  IF (LastPlayer = 0) THEN LastPlayer := NoPlayers;

  (* Control trump cards *)
  INC(Trump);
  IF (Trump = 5) THEN Trump := 1;

  (* Decide how many cards to deal *)
  IF (RoundNo > MaxCards) THEN
  BEGIN
    NoCards := 2*MaxCards - RoundNo;
  END
  ELSE
  BEGIN
    NoCards := RoundNo;
  END;
  Deal;

  (* Sort cards *)
  SortCards;

  (* Update cheat windows if they exist *)
  IF (CheatCards) THEN  CheatCardsPntr^.DrawScreen(0);

  (* Calculate initial probabilites for player one *)
  FOR A := 1 TO NoPlayers DO
  BEGIN
    FOR B := 1 TO 52 DO
      Probability[A, B] := (100 * NoCards) DIV 52;
  END;

  (* Update probability window if it exists *)
  IF (BoxExistProb) THEN  InvalidateRect(PntrProb^.HWindow, NIL, TRUE);

  (* Get players call no *)
  B := 1;
  IF (StartPlayer > 1) THEN
  BEGIN
    FOR A := StartPlayer TO NoPlayers DO
    BEGIN
      AutoCallTwo(A, B);
      INC(B);
    END;
  END;

  (* Draw cards et al *)
  IF (RoundNo = 1) THEN
    InvalidateRect(HWindow, NIL, TRUE)
  ELSE
  BEGIN
    DrawCards(0, TRUE);
    DrawPlayedCards(0, TRUE, 0);
    DrawInformation(0, 0);
  END;

  (* Get the human players call *)
  StatbarHan^.SetText(0, 'Indicate your call by clicking on the appropriate button');
  Application^.ExecDialog(New(PCallWin, Init(@self, 'CallWin')));
  StatbarHan^.SetText(0, '');
  INC(B);

  IF (StartPlayer = 1) THEN
  BEGIN
    FOR A := 2 TO NoPlayers DO
    BEGIN
      AutoCallTwo(A, B);
      INC(B);
    END;
  END
  ELSE
  BEGIN
    FOR A := 2 TO (StartPlayer - 1) DO
    BEGIN
      AutoCallTwo(A, B);
      INC(B);
    END;
  END;             

  (* Draw controls *)
  DrawInformation(0, 0);
END;


PROCEDURE TMainWindow.GameControlOne;
VAR
   A, B: INTEGER;
BEGIN
  (* Control is initially handed to here from "GameDeal" -  
     otherwise it is handed here from GameControlTwo. *)

  (* Set all players cards to be legal *)
  FOR A := 1 TO NoPlayers DO
  BEGIN
    FOR B := 1 TO MaxCards DO
      LegalHandCards[A, B] := 1;
  END;

  (* Play for the players who are to play BEFORE the human player *)
  IF (StartPlayer > 1) THEN
  BEGIN
    FOR PlayerToPlay := StartPlayer TO NoPlayers DO
    BEGIN
      IF (PlayerToPlay = StartPlayer) THEN
        PlayLead(PlayerToPlay)
      ELSE
      BEGIN
        PlayCard(PlayerToPlay);
      END;
    END;
  END;

  (* Update the display *)
  DrawPlayedCards(0, TRUE, 0); 

  (* Update cheat windows if they exist *)
  IF (CheatCards) THEN
    CheatCardsPntr^.DrawScreen(0);

  (* Now it is the turn for the human player to do his stuff *)
  PlayerToPlay := 1;

  (* Firstly find which cards the human player can legally play *)
  IF (StartPlayer <> 1) THEN ObtainLegal(1);
  DrawCards(0, FALSE);
  StatbarHan^.SetText(0, 'Select a card to play by clicking the left mouse button on it');
END;


PROCEDURE TMainWindow.GameControlTwo(Skip: BOOLEAN);
VAR
   A, B, C, D: INTEGER;
   TextString, TextStrtwo: ARRAY [0..30] OF CHAR;
BEGIN

  IF NOT(Skip) THEN
  BEGIN
    (* The human player has had his fun - now the automatic players finish *)
    IF (LastPlayer > 1) THEN
    BEGIN
      FOR PlayerToPlay := 2 TO LastPlayer DO
      BEGIN
        PlayCard(PlayerToPlay);
      END;
    END;

    (* Update the card display *)
    DrawPlayedCards(0, FALSE, 0);

    (* Update cheat windows if they exist *)
    IF (CheatCards) THEN
      CheatCardsPntr^.DrawScreen(0);

    (* Decide winner *)
    DecideWinner;

    (* Check to see if we should temporarily exit *)
    IF (NextCardNotify = Mouse) THEN
    BEGIN
      StatbarHan^.SetText(0, 'Click the left mouse button to continue');
      WaitForMouseClick := TRUE;
      EXIT;
    END;
  END;
  StatbarHan^.SetText(0, '');

  (* Correct number of cards per player *)       
  DEC(NoCards);

  (* Reset played card array *)
  FOR A := 1 TO NoPlayers DO
    CardPlayed[A] := 0;

  (* Update display *)
  DrawInformation(0, 0);
  DrawCards(0, FALSE);

  (* Start next trick/round *)
  IF (NoCards = 0) THEN
  BEGIN
    CalculateScore;
    DrawInformation(0, 0);
    IF (RoundNo = NumberofRounds) THEN
    BEGIN
      (* The game is over - decide who won *)
      C := 0;
      FOR A := 1 TO NoPlayers DO
      BEGIN
        IF (Score[A] > C) THEN
        BEGIN
          B := A;
          C := Score[A];
        END;
      END;

      (* Draw cards *)
      DrawCards(0, TRUE);
      DrawPlayedCards(0, TRUE, 0);

      IF (B = 1) THEN
        StrCopy(TextString, 'Well done! - You''ve won!')
      ELSE
      BEGIN
        StrCopy(TextString, 'Game won by player ');
        Str(B, TextStrtwo);
        StrCat(TextString, TextStrtwo);
      END;

      StatbarHan^.SetText(0, TextString);
      MessageBox(HWindow, TextString, 'Estimation Whist', MB_ICONINFORMATION);

      (* Decide if this is an new high score *)
      IF ((Score[1] > BestScores[10]) AND (B = 1)) THEN
      BEGIN
        StatbarHan^.SetText(0, 'Please enter your name in the dialog box');
        Application^.ExecDialog(New(PNameBox, Init(@self, 'Name')));
        StrCopy(TextString, 'Well done ');
        StrCat(TextString, NameInputText);
        StatbarHan^.SetText(0, TextString);
        A := 1;
        WHILE (Score[1] < BestScores[A]) DO
        BEGIN
          INC(A);
        END;

        FOR B := 10 DOWNTO (A+1) DO
        BEGIN
          BestScores[B] := BestScores[B-1];
          BestNames[B] := BestNames[B-1];
        END;

        BestScores[A] := Score[1];
        StrCopy(BestNames[A], NameInputText);
      END;

      (* Reset scores/calls *)
      FOR A := 1 TO NoPlayers DO    
      BEGIN
        Score[A] := 0;
        PlayCall[A] := 0;
      END;

      GameInProgress := FALSE;
      StatbarHan^.SetText(0, 'Estimation Whist - Version 1.2 - Copyright MG Davidson 1994-2002');

      (* Redraw the main window *)
      InvalidateRect(HWindow, NIL, TRUE);

      (* Reinstate random things if they previously existed *)
      IF (ExistRnd) THEN
        SetTimer(HWindow, ID_RndTimer, TimeRnd, nil);
    END
    ELSE
    BEGIN
      (* Start the next round *)
      StartRound;
      GameControlOne;
    END;
  END
  ELSE
  BEGIN
    (* Keep playing the same round *)
    GameControlOne;
  END;
END;


PROCEDURE TMainWindow.PlayCard(WhichPlayer: INTEGER);
VAR
   A: INTEGER;
BEGIN
  (* We use a random playing algorithm *)
  ObtainLegal(WhichPlayer);

  (* Get a legal card to play... *)
  REPEAT
    A := Random(MaxCards) + 1;
  UNTIL ((HandCards[WhichPlayer, A] > 0) AND (LegalHandCards[WhichPlayer, A] = 1)); 

  (* ...and then play it! *)
  CardPlayed[WhichPlayer] := HandCards[WhichPlayer, A];
  HandCards[WhichPlayer, A] := 0;
END;


PROCEDURE TMainWindow.PlayLead(WhichPlayer: INTEGER);
VAR
   A: INTEGER;
BEGIN
  (* Random playing style to start with *)
  REPEAT
    A := Random(MaxCards) + 1;
  UNTIL (HandCards[WhichPlayer, A] > 0);

  (* Effectively play any card that comes to hand *)
  CardPlayed[WhichPlayer] := HandCards[WhichPlayer, A];
  HandCards[WhichPlayer, A] := 0;
END;


PROCEDURE TMainWindow.ObtainLegal(GivenPlayer: INTEGER);
VAR
   SuitLed, A: INTEGER;
   CardFound: BOOLEAN;
BEGIN
  (* This procedure decides whether the given players hand cards are
  legal or not - ie what cards can he play *)

  (* Determine the suit of the led card *)
  SuitLed := DetermineSuit(CardPlayed[StartPlayer]);
  CardFound := FALSE;

  FOR A := 1 TO MaxCards DO
  BEGIN
    IF (DetermineSuit(HandCards[GivenPlayer, A]) <> SuitLed) THEN
      LegalHandCards[GivenPlayer, A] := 0
    ELSE
    BEGIN
      LegalHandCards[GivenPlayer, A] := 1;
      CardFound := TRUE;
    END;
  END;

  (* If no cards of the led suit then all cards in hand are valid... *)
  IF NOT(CardFound) THEN
  BEGIN
    FOR A := 1 TO MaxCards DO
    BEGIN
      LegalHandCards[GivenPlayer, A] := 1;
    END;
  END;
END;


PROCEDURE TMainWindow.CalculateScore;
VAR
   A: INTEGER;
BEGIN
  (* This procedure calculates all the scores at the end of a round *)
  FOR A := 1 TO NoPlayers DO
  BEGIN
    IF (ScoreMode = Vanilla) THEN
    BEGIN
      Score[A] := Score[A] + Tricks[A];
      IF (PlayCall[A] = Tricks[A]) THEN
        Score[A] := Score[A] + 10;
      Tricks[A] := 0;
    END;
    IF (ScoreMode = Squared) THEN
    BEGIN
      IF (PlayCall[A] = Tricks[A]) THEN
        Score[A] := Score[A] + 10 + (Tricks[A] * Tricks[A])
      ELSE
        Score[A] := Score[A] + Tricks[A];
      Tricks[A] := 0;
    END;
  END;
END;


PROCEDURE TMainWindow.DecideWinner;
VAR
   MaxPlayed, PlayerMax, TrumpUp, TrumpDown, Up, Down, Ace,
   A, B: INTEGER;
   PlayTemp: ARRAY [1..MaxPlayNo] OF INTEGER;
   TrumpCheck: BOOLEAN;
   TextString, TextStrtwo: ARRAY [0..40] OF CHAR;
BEGIN
  (* Decide the winner of the particular trick *)
  TrumpUp := Trump*13+1;
  TrumpDown := (Trump-1)*13;

  (* Copy played card values into a temporary array *)
  FOR A := 1 TO NoPlayers DO
    PlayTemp[A] := CardPlayed[A];

  (* Check to see if any trumps were played - if so consider only them *)
  TrumpCheck := FALSE;
  FOR A := 1 TO NoPlayers DO
  BEGIN
    IF ((CardPlayed[A] > TrumpDown) AND (CardPlayed[A] < TrumpUp)) THEN
      TrumpCheck := TRUE;
  END;

  IF (TrumpCheck) THEN
  BEGIN
    FOR A := 1 TO NoPlayers DO
    BEGIN
      IF NOT((CardPlayed[A] > TrumpDown) AND (CardPlayed[A] < TrumpUp)) THEN
        PlayTemp[A] := 0;
    END;
    Ace := TrumpDown + 1;
  END
  ELSE
  BEGIN
    MaxPlayed := PlayTemp[StartPlayer];
    IF ((MaxPlayed > 0) AND (MaxPlayed < 14)) THEN
    BEGIN
      Up := 14;
      Down := 0;
      Ace := 1;
    END;
    IF ((MaxPlayed > 13) AND (MaxPlayed < 27)) THEN
    BEGIN
      Up := 27;
      Down := 13;
      Ace := 14;
    END;
    IF ((MaxPlayed > 26) AND (MaxPlayed < 40)) THEN
    BEGIN
      Up := 40;
      Down := 26;
      Ace := 27;
    END;
    IF ((MaxPlayed > 39) AND (MaxPlayed < 53)) THEN
    BEGIN
      Up := 53;
      Down := 39;
      Ace := 40;
    END;

    FOR A := 1 TO NoPlayers DO
    BEGIN    
      IF NOT((PlayTemp[A] > Down) AND (PlayTemp[A] < Up)) THEN
        PlayTemp[A] := 0;
    END;
  END;

  (* The "PlayTemp" array should now consist solely of Trump cards or
     entirely of cards of the led suit and "Ace" should contain the
     value of the associated ace card. *)

  MaxPlayed := PlayTemp[1];
  IF (MaxPlayed = Ace) THEN MaxPlayed := 60;
  PlayerMax := 1;

  FOR A := 2 TO NoPlayers DO
  BEGIN
    IF (PlayTemp[A] <> Ace) THEN
    BEGIN
      IF (PlayTemp[A] > MaxPlayed) THEN
      BEGIN
        MaxPlayed := PlayTemp[A];
        PlayerMax := A;
      END;
    END
    ELSE
    BEGIN
      MaxPlayed := 60;
      PlayerMax := A;
    END;
  END;

  (* Give the necessary credit *) 
  INC(Tricks[PlayerMax]);
  StartPlayer := PlayerMax;
  LastPlayer := PlayerMax - 1;
  IF (LastPlayer = 0) THEN LastPlayer := NoPlayers;

  StrCopy(TextString, 'Player ');
  Str(PlayerMax, TextStrtwo);
  StrCat(TextString, TextStrtwo);
  StrCat(TextString, ' won that trick. ');
  DrawInformation(0, PlayerMax);
  IF (NextCardNotify = Dialog) THEN
    MessageBox(HWindow, TextString, 'Estimation Whist', mb_IconInformation);
END;


PROCEDURE TMainWindow.AutoCall(PlayNo, Place: INTEGER);
VAR
   A, B, C: INTEGER;
   RA: REAL;
BEGIN
  (* This procedure decides what is a reasonable call for each of the
     automatic players. It uses the function CallUa where :-

     PlayNo : Player number
     Place  : Playing position (ie first, second)
  *)

  (* Fill selection array with zero's *)
  FOR A := 1 TO 4 DO
  BEGIN
    FOR B := 1 TO 5 DO
      Sel[A, B] := 0;
  END;

  TempPlayCall[PlayNo] := 0.0;

  (* Check to see what the hand contains *)
  FOR A := 0 TO 3 DO
  BEGIN
    FOR B := 1 TO MaxCards DO
    BEGIN
      C := A * 13;
      IF (HandCards[PlayNo, B] = (C+1)) THEN Sel[(A+1), 1] := 1; 
      IF (HandCards[PlayNo, B] = (C+13)) THEN Sel[(A+1), 2] := 1;
      IF (HandCards[PlayNo, B] = (C+12)) THEN Sel[(A+1), 3] := 1;
      IF (HandCards[PlayNo, B] = (C+11)) THEN Sel[(A+1), 4] := 1;
      IF ((HandCards[PlayNo, B] > (C+1)) AND
          (HandCards[PlayNo, B] < (C+11))) THEN
        INC(Sel[(A+1), 5]);
    END;
  END;

  (* Decide what to call *)
  (* Do the trump suit first *)
  IF ((Sel[Trump, 1] = 1) AND (Sel[Trump, 2] = 1) AND (Sel[Trump, 3] = 1)
                          AND (Sel[Trump, 4] = 1)) THEN
    FOR A := 1 TO 4 DO CallUa(Trump, A, PlayNo, 1.0);
  IF ((Sel[Trump, 2] = 1) AND (Sel[Trump, 3] = 1)) THEN
  BEGIN
    CallUa(Trump, 2, PlayNo, 1.0);
    CallUa(Trump, 3, PlayNo, 1.0);
    IF (Sel[Trump, 5] > 1) THEN
      CallUa(Trump, 5, PlayNo, 1.0);
  END;  
  IF ((Sel[Trump, 3] = 1) AND (Sel[Trump, 4] = 1)) THEN
  BEGIN
    CallUa(Trump, 3, PlayNo, 1.0);
    CallUa(Trump, 4, PlayNo, 0.0);
    IF (Sel[Trump, 5] > 1) THEN
      CallUa(Trump, 5, PlayNo, 1.0);
  END;
  IF ((Sel[Trump, 2] = 1) AND (Sel[Trump, 4] = 1)) THEN
  BEGIN
    CallUa(Trump, 2, PlayNo, 1.0);
    CallUa(Trump, 4, PlayNo, 0.0);
    IF (Sel[Trump, 5] > 1) THEN
      CallUa(Trump, 5, PlayNo, 1.0);
  END;
  IF (Sel[Trump, 1] = 1) THEN
    CallUa(Trump, 1, PlayNo, 1.0);
  IF (Sel[Trump, 2] = 2) THEN 
    CallUa(Trump, 2, PlayNo, 1.0);
  IF (Sel[Trump, 3] = 3) THEN
    CallUa(Trump, 3, PlayNo, 1.0);
  IF ((Sel[Trump, 5] > 1) AND (TempPlayCall[PlayNo] > 0.0)) THEN
  BEGIN
    RA := Sel[Trump, 5] / 2.0;
    CallUa(Trump, 5, PlayNo, RA);
  END;
  IF (Sel[Trump, 5] > 2) THEN
  BEGIN
    RA := Sel[Trump, 5] / 2.0;
    CallUa(Trump, 5, PlayNo, RA);
  END;

  (* Now for the other suits *)
  FOR A := (Trump + 1) TO (Trump + 3) DO
  BEGIN
    B := A;
    IF (B > 4) THEN B := B - 4;

    IF ((Sel[B, 1] = 1) AND (Sel[B, 2] = 1) AND (Sel[B, 3] = 1)) THEN
    BEGIN
      CallUa(B, 1, PlayNo, 2.5);
      CallUa(B, 2, PlayNo, 0.0);
      CallUa(B, 2, PlayNo, 0.0);
    END;
    IF ((Sel[B, 2] = 1) AND (Sel[B, 3] = 1)) THEN
    BEGIN
      CallUa(B, 2, PlayNo, 1.0);
      CallUa(B, 3, PlayNo, 0.0);
      IF (Sel[B, 5] > 1) THEN CallUa(B, 5, PlayNo, 1.0);
    END; 
    IF ((Sel[B, 2] = 1) AND (Sel[B, 4] = 1)) THEN
    BEGIN
      CallUa(B, 2, PlayNo, 1.0);
      CallUa(B, 3, PlayNo, 0.0);
      IF (Sel[B, 5] > 1) THEN CallUa(B, 5, PlayNo, 1.0);
    END;
    IF ((Sel[B, 3] = 1) AND (Sel[B, 4] = 1)) THEN
    BEGIN
      CallUa(B, 3, PlayNo, 0.5);
      CallUa(B, 4, PlayNo, 0.0);
      IF (Sel[B, 5] > 1) THEN CallUa(B, 5, PlayNo, 0.5);
    END;
    IF (Sel[B, 1] = 1) THEN
      CallUa(B, 1, PlayNo, 1.0);
    IF (Sel[B, 2] = 1) THEN
      CallUa(B, 2, PlayNo, 1.0);

  END;

  (* We now have the "ideal" call - but wait - check to see what the others
     are doing (this is, of course, perfectly legal) *)

  B := 0;
  FOR A := 1 TO NoPlayers DO
  BEGIN
    B := B + PlayCall[A];  (* B now contains the total number of tricks called so far *)
  END;

  C := NoCards DIV NoPlayers; (* C now contains rough average value of tricks that should
                                 be called per player *)

  IF ((NoCards > 3) AND ((B DIV Place) < (C + 2))) THEN
  BEGIN
    TempPlayCall[PlayNo] := TempPlayCall[PlayNo] + 1 + 0.4*((B DIV Place) - (C+2));
  END; 

  IF (Place = NoPlayers) THEN
  BEGIN
    IF ((B + TempPlayCall[PlayNo]) = NoCards) THEN
      TempPlayCall[PlayNo] := TempPlayCall[PlayNo] - 1;
  END;

  PlayCall[PlayNo] := ROUND(TempPlayCall[PlayNo]);

  IF (PlayCall[PlayNo] < 0) THEN PlayCall[PlayNo] := 0;

END;


PROCEDURE TMainWindow.AutoCallTwo(PlayNo, Place: INTEGER);
VAR
  SumCalls, A, B: INTEGER;
  TempCall: REAL;
BEGIN
  (*  This routine calls for the players using various card value arrays,
      each card being assigned a predetermined value. These values are then
      summed and rounded up to give the overall call. The values are split
      into three groups dependant on the number of cards in the hand. These
      are :-

      1) One card in the hand
      2) Two to five cards
      3) Six cards and above


      Note that the trump card is numbered 1 to 4 and that the hand cards
      are numbered 1 to 52 where 1 is the ace of clubs, and 13 is the King
      of clubs.

      Two arrays ("CardVal" and "TrumpCardVal") are maintained for cards in
      the trump and non-trump suits. The values in these arrays determine the
      call value. The values in these arrays are set by two procedures
      "CallValueOne" and "CallValueTwo". An integer flag is maintained which
      identifies which setting currently prevails. This flag is known as
      "CardValSetting". The values mean :-

      0) - Not set: the values in the arrays may be incorrect or zero.
      1) - Values reflect settings as stored in procedure "CallValueOne".
      2) - Values reflect settings as stored in procedure "CallValueTwo". 
   *)

  TempCall := 0;                (* Reset players previous temporary call *)
  SumCalls := 0;                (* Reset sum of total calls made so far *)
  FOR A := 1 TO NoPlayers DO    (* Calculate sum of total calls made so far *)
    SumCalls := SumCalls + PlayCall[A];

  IF (NoCards = 1) THEN
  BEGIN
    IF (HandCards[PlayNo, 1] = (((Trump - 1) * 13) + 1)) THEN
    BEGIN
      (* Call one as card must take a trick *)
      TempCall := 1;
    END
    ELSE
    BEGIN
      IF (SumCalls > 0) THEN
      BEGIN
        (* Call nothing as it is unlikely that we will get a trick *)
        TempCall := 0;
      END
      ELSE
      BEGIN
        (* Check if we have a trump - if it is greater than 5
        then call one otherwise call zero *)
        IF ((HandCards[PlayNo, 1] > (((Trump - 1) * 13) + 5)) AND
            (HandCards[PlayNo, 1] < ((Trump * 13) + 1))) THEN
          TempCall := 1
        ELSE
          TempCall := 0;
      END;
    END;
  END;

  IF ((NoCards > 1) AND (NoCards < 6)) THEN
  BEGIN
    (* If the correct card value data is not currently in the array then load it *)
    IF (CardValSetting <> 1) THEN
      CallValueOne;

    FOR A := 1 TO MaxCards DO
    BEGIN
      B := HandCards[PlayNo, A];
      IF (B > 0) THEN
      BEGIN
        (* Is it a trump card? *)
        IF ((B > ((Trump - 1) * 13)) AND
            (B < ((Trump * 13) + 1))) THEN
        BEGIN
          (* Yes, it is *)
          WHILE (B > 13) DO
          BEGIN
            B := B - 13;
          END;

          TempCall := TempCall + TrumpCardVal[B];
        END
        ELSE
        BEGIN
          (* No, it's not *)
          WHILE (B > 13) DO
          BEGIN
            B := B - 13;
          END;

          TempCall := TempCall + CardVal[B];
        END;
      END;
    END;

  END;

  IF (NoCards > 5) THEN
  BEGIN
    (* If the correct card value data is not currently in the array then load it *)
    IF (CardValSetting <> 2) THEN
      CallValueTwo;

    FOR A := 1 TO MaxCards DO
    BEGIN
      B := HandCards[PlayNo, A];
      IF (B > 0) THEN
      BEGIN
        (* Is it a trump card? *)
        IF ((B > ((Trump - 1) * 13)) AND
            (B < ((Trump * 13) + 1))) THEN
        BEGIN
          (* Yes, it is *)
          WHILE (B > 13) DO
          BEGIN
            B := B - 13;
          END;

          TempCall := TempCall + TrumpCardVal[B];
        END
        ELSE
        BEGIN
          (* No, it's not *)
          WHILE (B > 13) DO
          BEGIN
            B := B - 13;
          END;

          TempCall := TempCall + CardVal[B];
        END;
      END;
    END;
  END;

  (* Add a half and then truncate - equivalent to correct rounding *)
  PlayCall[PlayNo] := TRUNC(TempCall + 0.5); 

  (* If last player tries to call correct number of tricks then round down *)
  IF (Place = NoPlayers) THEN
  BEGIN
    IF ((SumCalls + PlayCall[PlayNo]) = NoCards) THEN
    BEGIN
      IF (PlayCall[PlayNo] = 0) THEN
        PlayCall[PlayNo] := 1
      ELSE
        PlayCall[PlayNo] := PlayCall[PlayNo] - 1;
    END;
  END;

END;


PROCEDURE TMainWindow.CallValueOne;
BEGIN
  (* This procedure sets the arrays "CardVal" and "TrumpCardVal" to
     specified values. It also sets the "CardValSetting" flag to 1.
  *)

  CardVal[1]  := 1.0;   (* Ace *)
  CardVal[2]  := 0.0;   (* 2 *)
  CardVal[3]  := 0.0;   (* 3 *)
  CardVal[4]  := 0.0;   (* 4 *)
  CardVal[5]  := 0.0;   (* 5 *)
  CardVal[6]  := 0.0;   (* 6 *)
  CardVal[7]  := 0.0;   (* 7 *)
  CardVal[8]  := 0.1;   (* 8 *)
  CardVal[9]  := 0.2;   (* 9 *)
  CardVal[10] := 0.3;   (* 10 *)
  CardVal[11] := 0.4;   (* Jack *)
  CardVal[12] := 0.5;   (* Queen *)
  CardVal[13] := 0.8;   (* King *)

  TrumpCardVal[1]  := 1.0;  (* Ace *)
  TrumpCardVal[2]  := 0.1;  (* 2 *)
  TrumpCardVal[3]  := 0.2;  (* 3 *)
  TrumpCardVal[4]  := 0.2;  (* 4 *)
  TrumpCardVal[5]  := 0.3;  (* 5 *)
  TrumpCardVal[6]  := 0.3;  (* 6 *)
  TrumpCardVal[7]  := 0.3;  (* 7 *)
  TrumpCardVal[8]  := 0.4;  (* 8 *)
  TrumpCardVal[9]  := 0.7;  (* 9 *)
  TrumpCardVal[10] := 0.9;  (* 10 *)
  TrumpCardVal[11] := 1.0;  (* Jack *)
  TrumpCardVal[12] := 1.0;  (* Queen *)
  TrumpCardVal[13] := 1.0;  (* King *)

  CardValSetting := 1;
END;


PROCEDURE TMainWindow.CallValueTwo;
BEGIN
  (* This procedure sets the arrays "CardVal" and "TrumpCardVal" to
     specified values. It also sets the "CardValSetting" flag to 2.
  *)

  CardVal[1]  := 1.0;  (* Ace *)
  CardVal[2]  := 0.0;  (* 2 *)
  CardVal[3]  := 0.0;  (* 3 *)
  CardVal[4]  := 0.0;  (* 4 *)
  CardVal[5]  := 0.0;  (* 5 *)
  CardVal[6]  := 0.0;  (* 6 *)
  CardVal[7]  := 0.1;  (* 7 *)
  CardVal[8]  := 0.1;  (* 8 *)
  CardVal[9]  := 0.1;  (* 9 *)
  CardVal[10] := 0.1;  (* 10 *)
  CardVal[11] := 0.1;  (* Jack *)
  CardVal[12] := 0.4;  (* Queen *)
  CardVal[13] := 1.0;  (* King *)

  TrumpCardVal[1]  := 1.0;  (* Ace *)
  TrumpCardVal[2]  := 0.1;  (* 2 *)
  TrumpCardVal[3]  := 0.1;  (* 3 *)
  TrumpCardVal[4]  := 0.1;  (* 4 *)
  TrumpCardVal[5]  := 0.1;  (* 5 *)
  TrumpCardVal[6]  := 0.2;  (* 6 *)
  TrumpCardVal[7]  := 0.2;  (* 7 *)
  TrumpCardVal[8]  := 0.3;  (* 8 *)
  TrumpCardVal[9]  := 0.3;  (* 9 *)
  TrumpCardVal[10] := 0.4;  (* 10 *)
  TrumpCardVal[11] := 0.5;  (* Jack *)
  TrumpCardVal[12] := 0.9;  (* Queen *)
  TrumpCardVal[13] := 1.0;  (* King *)

  CardValSetting := 2;
END;


PROCEDURE TMainWindow.CallUa(Suit, CardNo, Play: INTEGER; Add: REAL);
BEGIN
  (* Used to calculate automatic play call *)
  TempPlayCall[Play] := TempPlayCall[Play] + Add;
  IF (CardNo = 5) THEN
  BEGIN
    DEC(Sel[Suit, 5]);
    IF (Sel[Suit, 5] < 0) THEN Sel[Suit, 5] := 0;
  END
  ELSE
    Sel[Suit, 5] := 0; 
END;


PROCEDURE TMainWindow.Deal;
VAR
   A, B: INTEGER;
BEGIN
  (* Reset every players' hand *)
  FOR A := 1 TO MaxPlayNo DO
  BEGIN
    FOR B := 1 TO MaxCards DO
      HandCards[A, B] := 0;
  END;

  (* Deal "NoCards" to each player up to "NoPlayers" *)
  StackPntr := 1;
  FOR A := 1 TO NoPlayers DO
  BEGIN
    FOR B := 1 TO NoCards DO
    BEGIN
      HandCards[A, B] := Cards[StackPntr];
      INC(StackPntr);
    END;
  END;
END;


PROCEDURE TMainWindow.Shuffle;
VAR
   CardPosA, CardPosB, Temp, X: INTEGER;
BEGIN

  (* Fill card array *)
  FOR X := 1 TO 52 DO
    Cards[X] := X;

  (* Shuffle card array *)
  FOR X := 1 TO 1000 DO
  BEGIN
    (* Generate a random position *)
    CardPosA := RANDOM(52) + 1;
    CardPosB := RANDOM(52) + 1;

    (* Swap the cards about *)
    Temp := Cards[CardPosA];
    Cards[CardPosA] := Cards[CardPosB];
    Cards[CardPosB] := Temp;
  END;
END;


PROCEDURE TMainWindow.SortCards;
VAR
   A, B, C, Top, MaxCard, MaxPos: INTEGER;
BEGIN
  FOR A := 1 TO NoPlayers DO
  BEGIN
    Top := NoCards;
    FOR B := 1 TO (NoCards - 1)  DO
    BEGIN
      MaxCard := HandCards[A, 1];
      MaxPos := 1;
      FOR C := 2 TO Top DO
      BEGIN
        IF (HandCards[A, C] > MaxCard) THEN
        BEGIN
          MaxCard := HandCards[A, C];
          MaxPos := C;
        END;
      END;

      (* Swap cards about *)
      MaxCard := HandCards[A, MaxPos];
      HandCards[A, MaxPos] := HandCards[A, Top];
      HandCards[A, Top] := MaxCard;

      DEC(Top);
    END;
  END;
END;


PROCEDURE TMainWindow.DrawCards(DC: HDC; Redraw: BOOLEAN);
VAR
   MadeDC: BOOLEAN;
   Rectangle: TRect;
   A, B, C, ActWidth, LastDrawn: INTEGER;
BEGIN
  (* Check to see if window is iconised *)
  IF (IsIconic(HWindow)) THEN EXIT;

  (* Check to see if passed a DC *)
  IF (DC = 0) THEN
  BEGIN
    (* Otherwise create new DC *)
    DC := GetDC(HWindow);
    MadeDC := True;
  END
  ELSE
    MadeDC := False;

  (* Clear screen area where cards are to be drawn *)
  IF (ReDraw) THEN
  BEGIN
    WITH Rectangle DO
    BEGIN
      Left := 0;
      Right := 510;
      Top := 229;
      Bottom := CardHeight + 234;
    END;
    FillRect(DC, Rectangle, GreenBrush);
  END;

  (* Initialise position array *)
  FOR A := 1 TO MaxCards DO
  BEGIN
    WITH CardPos[A] DO
    BEGIN
      Left := 0;
      Right := 0;
      Top := 0;
      Bottom := 0;
    END;
  END;

  (* Recalculate NoCards to make sure it is OK *)
  NoCards := 0;
  FOR A := 1 TO MaxCards DO
  BEGIN
    IF (HandCards[1, A] > 0) THEN
      INC(NoCards);
  END; 

  (* Draws cards dependant on the window size *)
  LastDrawn := 0;
  C := 1;
  IF ((NoCards - 1) > 0) THEN
  BEGIN
    ActWidth := (500 - CardWidth) DIV (NoCards - 1);
    IF (ActWidth > MinWidth) THEN
    BEGIN
      IF (ActWidth > (CardWidth + 10)) THEN ActWidth := CardWidth + 10;
      FOR B := 1 TO MaxCards DO
      BEGIN
        IF (HandCards[1, B] > 0) THEN
        BEGIN
          IF (LegalHandCards[1, B] = 1) THEN
          BEGIN
            DrawBMP(DC, 10+(C-1)*ActWidth, 234, CardPic[HandCards[1, B]],
                    FALSE);
          END
          ELSE
            DrawBMP(DC, 10+(C-1)*ActWidth, 234, CardPic[HandCards[1, B]],
                    TRUE);
          BEGIN
          END;
          WITH CardPos[B] DO
          BEGIN
            Left := 10+(C-1)*ActWidth;
            Right := Left + CardWidth;
            Top := 234;
            Bottom := Top + CardHeight;
          END;
          IF ((LastDrawn > 0)  AND (ActWidth < CardWidth)) THEN
            CardPos[LastDrawn].Right := CardPos[LastDrawn].Left + ActWidth;
          LastDrawn := B;
          INC(C);
        END
        ELSE
        BEGIN
          WITH CardPos[B] DO
          BEGIN
            Left := 0;
            Right := 0;
            Top := 0;
            Bottom := 0;
          END;
        END;
      END;
    END
    ELSE
    BEGIN
      MessageBox(HWindow, 'Window too small to draw cards',
                 'Estimation Whist', mb_IconInformation);
    END;
  END
  ELSE
  BEGIN
    FOR A := 1 TO MaxCards DO
    BEGIN
      IF (HandCards[1, A] > 0) THEN
      BEGIN
        IF (LegalHandCards[1, A] = 1) THEN
        BEGIN
          DrawBMP(DC, 10, 234, CardPic[HandCards[1, A]], FALSE);
        END
        ELSE
        BEGIN
          DrawBMP(DC, 10, 234, CardPic[HandCards[1, A]], TRUE);
        END;
        WITH CardPos[A] DO
        BEGIN
          Left := 10;
          Right := Left + CardWidth;
          Top := 234;
          Bottom := Top + CardHeight;
        END;
      END;
    END;
  END;

  (* Release device context *)
  IF (MadeDC) THEN ReleaseDC(HWindow, DC);
END;


PROCEDURE TMainWindow.DrawPlayedCards(DC: HDC; Redraw: BOOLEAN; Player: INTEGER);
VAR
   MadeDC: BOOLEAN;
   Rectangle: TRect;
   A, B: INTEGER;
   TextString: ARRAY [0..5] OF CHAR;
BEGIN

  (* Check to see if passed a DC *)
  IF (DC = 0) THEN
  BEGIN
    (* Otherwise create new DC *)
    DC := GetDC(HWindow);
    MadeDC := TRUE;
  END
  ELSE
    MadeDC := FALSE;

  (* Clear screen area where cards are to be drawn *)
  IF (Redraw) THEN
  BEGIN
    WITH Rectangle DO
    BEGIN
      Left := 39;
      Right := CardWidth + 41 + (NoPlayers-1)*30;
      Top := 54;
      Bottom := CardHeight + 80;
    END;
    FillRect(DC, Rectangle, GreenBrush);
  END;

  (* Draw the played cards *)
  IF (Player = 0) THEN
  BEGIN
    B := StartPlayer;
    IF (B > 0) THEN
    BEGIN
      FOR A := 1 TO NoPlayers DO
      BEGIN
        SetTextColor(DC, $00000000);
        IF (CardPlayed[B] > 0) THEN
        BEGIN
          DrawBMP(DC, (10+30*A), 55, CardPic[CardPlayed[B]], FALSE);
          SetBkColor(DC, $00008000);
          Str(B, TextString);
          TextOut(DC, (20+30*A), (65+CardHeight), TextString,
                  StrLen(TextString));
          SetBkColor(DC, $00FFFFFF);
        END;
        INC(B);
        IF (B > NoPlayers) THEN B := 1;    
      END;
    END;
  END
  ELSE
  BEGIN
    IF (CardPlayed[Player] > 0) THEN
    BEGIN
      DrawBmp(DC, (10+30*Player), 55, CardPic[CardPlayed[Player]], FALSE);
      SetBkColor(DC, $00008000);
      SetBkColor(DC, $00008000);
      Str(Player, TextString);
      TextOut(DC, (20+30*Player), (65+CardHeight), TextString,
              StrLen(TextString));
      SetBkColor(DC, $00FFFFFF);
    END;
  END;

  (* Tidy up *)
  IF (MadeDC) THEN ReleaseDC(HWindow, DC);
END;


PROCEDURE TMainWindow.DrawBMP(DC: HDC; X, Y, BitMap: HBitmap; INV: BOOLEAN);
VAR
  MemDC: HDC;
  MadeDC: BOOLEAN;
BEGIN

  (* Check to see if passed a DC *)
  IF (DC = 0) THEN
  BEGIN
    (* Otherwise create new DC *)
    DC := GetDC(HWindow);
    MadeDC := True;
  END
  ELSE
    MadeDC := False;

  (* Guts of moving procedure *)
  MemDC := CreateCompatibleDC(DC);
  SelectObject(MemDC, BitMap);
  IF (INV) THEN
    BitBlt(DC, X, Y, CardWidth, CardHeight, MemDC, 0, 0, NotSRCCopy)
  ELSE
    BitBlt(DC, X, Y, CardWidth, CardHeight, MemDC, 0, 0, SRCCopy);
  DeleteDC(MemDC);

  (* Tidy up *)
  IF (MadeDC) THEN ReleaseDC(HWindow, DC);
END;


PROCEDURE TMainWindow.DrawSmallBMP(DC: HDC; X, Y: INTEGER; BitMap: HBitmap; Style: INTEGER);
VAR
  MemDC: HDC;
  MadeDC: BOOLEAN;
BEGIN

  (* Check to see if passed a DC *)
  IF (DC = 0) THEN
  BEGIN
    (* Otherwise create new DC *)
    DC := GetDC(HWindow);
    MadeDC := True;
  END
  ELSE
    MadeDC := False;

  (* Guts of moving procedure *)
  MemDC := CreateCompatibleDC(DC);
  SelectObject(MemDC, BitMap);
  CASE Style OF
    1 : BitBlt(DC, X, Y, SmallWidth, SmallHeight, MemDC, 0, 0, SRCCopy);
    2 : BitBlt(DC, X, Y, SmallWidth, SmallHeight, MemDC, 0, 0, SRCErase);
  END;

  DeleteDC(MemDC);

  (* Tidy up *)
  IF (MadeDC) THEN ReleaseDC(HWindow, DC);
END;


PROCEDURE TMainWindow.DrawControls(DC: HDC);
VAR
  TextString, TextStrtwo: ARRAY [0..100] OF CHAR;
  TheRect: TRECT;
  A: INTEGER;
  MadeDC: BOOLEAN;
BEGIN

  (* Check to see if passed a DC *)
  IF (DC = 0) THEN
  BEGIN
    (* Otherwise create new DC *)
    DC := GetDC(HWindow);
    MadeDC := True;
  END
  ELSE
    MadeDC := False;

  (* Draw controls on the screen *)
  IF (GameInProgress) THEN
  BEGIN
    (* Draw boxes to put the data into *)
    DrawBox(DC, 340, 20, 585, (60+15*NoPlayers));

    (* Draw call/tricks/score data *)
    SetTextColor(DC, $00000000);
    SetBkColor(DC, GetSysColor(COLOR_BTNFACE));
    StrCopy(TextString, 'Calls:');
    TextOut(DC, 420, 30, TextString, StrLen(TextString));
    StrCopy(TextString, 'Tricks:');
    TextOut(DC, 470, 30, TextString, StrLen(TextString));
    StrCopy(TextString, 'Scores:');
    TextOut(DC, 520, 30, TextString, StrLen(TextString));
    FOR A := 1 TO NoPlayers DO
    BEGIN
      StrCopy(TextString, 'Player ');
      Str(A, TextStrtwo);
      StrCat(TextString, TextStrtwo);
      StrCat(TextString, ':');    
      TextOut(DC, 350, (35+15*A), TextString, StrLen(TextString));
    END;

    (* Draw other details *)
    DrawBox(DC, 20, 198, 130, 225);
    StrCopy(TextString, 'Your hand: ');
    TextOut(DC, 30, 204, TextString, StrLen(TextString));
    DrawBox(DC, 20, 20, 130, 47);
    StrCopy(TextString, 'Played cards: ');
    TextOut(DC, 30, 26, TextString, StrLen(TextString));

    (* Other information *)
    DrawBox(DC, 410, 160, 585, 225);
  END
  ELSE
  BEGIN
    (* Clear stuff on RHS of screen... *)
    TheRect.Left := 340;
    TheRect.Top := 20;
    TheRect.Right := 585;
    TheRect.Bottom := 215;
    FillRect(DC, TheRect, GreenBrush);

    (* ..and then stuff on LHS *)
    TheRect.Left := 20;
    TheRect.Top := 10;
    TheRect.Right := 130;
    TheRect.Bottom := 215;
    FillRect(DC, TheRect, GreenBrush);
  END;

  (* Release DC *)
  IF (MadeDC) THEN ReleaseDC(HWindow, DC);
END;


PROCEDURE TMainWindow.DrawInformation(DC: HDC; Winner: INTEGER);
VAR
  TextString, TextStrtwo: ARRAY [0..100] OF CHAR;
  A: INTEGER;
  MadeDC: BOOLEAN;
BEGIN
  (* This procedure draws all the stuff that fills the text boxes on the screen *)
  IF (GameInProgress) THEN
  BEGIN
    (* Check to see if passed a DC *)
    IF (DC = 0) THEN
    BEGIN
      (* Otherwise create new DC *)
      DC := GetDC(HWindow);
      MadeDC := True;
    END
    ELSE
      MadeDC := False;
  
    (* Set up background colours *)
    SetBkColor(DC, GetSysColor(COLOR_BTNFACE));
    SetTextColor(DC, GetSysColor(COLOR_MENUTEXT));

    (* Output actual values *)
    FOR A := 1 TO NoPlayers DO
    BEGIN
      Str(PlayCall[A], TextString);
      StrCat(TextString, '    ');
      TextOut(DC, 430, (35 + 15*A), TextString, StrLen(TextString));
      Str(Tricks[A], TextString);
      StrCat(TextString, '    ');
      TextOut(DC, 490, (35 + 15*A), TextString, StrLen(TextString));
      Str(Score[A], TextString);
      StrCat(TextString, ' ');
      TextOut(DC, 540, (35 + 15*A), TextString, StrLen(TextString));
    END;

    (* Round and player information *)
    StrCopy(TextString, 'Round number: ');
    Str(RoundNo, TextStrtwo);
    StrCat(TextString, TextStrtwo);
    StrCat(TextString, ' of ');
    Str(NumberofRounds, TextStrtwo);
    StrCat(TextString, TextStrtwo);
    TextOut(DC, 420, 170, TextString, StrLen(TextString));
    StrCopy(TextString, 'Player to start: ');
    Str(RoundStartPlayer, TextStrtwo);
    StrCat(TextString, TextStrtwo);
    TextOut(DC, 420, 185, TextString, StrLen(TextString));

    (* Who won the last trick? *)
    IF (Winner > 0) THEN LastWinner := Winner;
    StrCopy(TextString, 'Last trick won by: ');
    IF (LastWinner > 0) THEN
      Str(LastWinner, TextStrtwo)
    ELSE
      StrCopy(TextStrtwo, '      ');
    StrCat(TextString, TextStrtwo);
    TextOut(DC, 420, 200, TextString, StrLen(TextString));

    (* Draw trump suit indicator *)
    SetBkColor(DC, $00FFFFFFF);
    CASE Trump OF
      1 : DrawSmallBMP(DC, 285, 80, ClubPic, 1);
      2 : DrawSmallBMP(DC, 285, 80, DiamPic, 1);
      3 : DrawSmallBMP(DC, 285, 80, HertPic, 1);
      4 : DrawSmallBMP(DC, 285, 80, SpdePic, 1);
    END;

    (* Release DC *)
    IF (MadeDC) THEN ReleaseDC(HWindow, DC);
  END;
END;


PROCEDURE TMainWindow.DrawBox(TheDC: HDC; X1, Y1, X2, Y2: INTEGER);
BEGIN
  (* Is the DC ok? *)
  IF (TheDC = 0) THEN EXIT;

  (* Draw a 3D box with an outline *)
  SelectObject(TheDC, GetStockObject(BLACK_PEN));
  SelectObject(TheDC, LtGrayBrush);
  Rectangle(TheDC, X1, Y1, X2, Y2);
  SelectObject(TheDC, DkGrayPen);
  MoveTo(TheDC, X1+5, Y2-5);
  LineTo(TheDC, X1+5, Y1+5);
  LineTo(TheDC, X2-5, Y1+5);
  SelectObject(TheDC, WhitePen);
  LineTo(TheDC, X2-5, Y2-5);
  LineTo(TheDC, X1+5, Y2-5);
END;


PROCEDURE TMainWindow.SetPosInd(DC: HDC);
VAR
   MadeDC: BOOLEAN;
   TextString: ARRAY [0..10] OF CHAR;
BEGIN

  (* Check to see if passed a DC *)
  IF (DC = 0) THEN
  BEGIN
    (* Otherwise create new DC *)
    DC := GetDC(HWindow);
    MadeDC := True;
  END
  ELSE
    MadeDC := False;

  (* This procedure sets the player position indicator *)
  IF ((PreviousPlayer < 1) OR (PreviousPlayer > NoPlayers)) THEN
    PreviousPlayer := PlayerToPlay;
  SetBkColor(DC, $00008000);
  SetTextColor(DC, $00000000);
  StrCopy(TextString, '    ');
  TextOut(DC, 330, (45+15*PreviousPlayer), TextString, StrLen(TextString));
  StrCopy(TextString, ' -> ');
  TextOut(DC, 330, (45+15*PlayerToPlay), TextString, StrLen(TextString));
  PreviousPlayer := PlayerToPlay;

  (* Release DC *)
  IF (MadeDC) THEN ReleaseDC(HWindow, DC);
END;


FUNCTION TMainWindow.CanClose: BOOLEAN;
VAR
  ReturnVal: BOOL;
BEGIN
  (* If the window can close then return true *)
  IF (GameInProgress) THEN
  BEGIN
    ReturnVal := TRUE;
    IF (ConfirmExit) THEN
      ReturnVal := BOOL(MessageBox(HWindow,
                                   'Abandon current game and exit?',
                                   'Estimation Whist',
                                   MB_YESNO OR MB_ICONQUESTION) = ID_YES);
    IF (ReturnVal) THEN
    BEGIN
      (* Close all child windows *)
      IF (BoxExistProb) THEN
        PntrProb^.CloseWindow;
      IF (CheatCards) THEN
        CheatCardsPntr^.CloseWindow;
      CanClose := TRUE;
    END
    ELSE
      CanClose := FALSE;
  END
  ELSE
  BEGIN
    (* Close all child windows *)
    IF (BoxExistProb) THEN
    BEGIN
      PntrProb^.CloseWindow;
      BoxExistProb := TRUE;
    END;

    IF (CheatCards) THEN
    BEGIN
      CheatCardsPntr^.CloseWindow;
      CheatCards := TRUE;
    END;

    CanClose := TRUE;
  END;
END;


FUNCTION TMainWindow.DetermineSuit(GivenVal: INTEGER): INTEGER;
VAR
   Temp: INTEGER;
BEGIN
  CASE (GivenVal) OF
     1..13 : Temp := 1;
    14..26 : Temp := 2;
    27..39 : Temp := 3;
    40..52 : Temp := 4;
  END;

  (* Return correct suit value *)
  DetermineSuit := Temp;
END;


FUNCTION TMainWindow.GetClassName: PChar;
BEGIN
  GetClassName := 'Estimation Whist window';
END;

(************************************************************************)
(* Methods of TProbWin follow... *)

CONSTRUCTOR TProbWin.Init(AParent: PWindowsObject; AName: PChar);
BEGIN
  (* Call ancestor method *)
  TWindow.Init(AParent, AName);

  (* Set up window *)
  WITH Attr DO
  BEGIN
    Style := Style OR WS_THICKFRAME OR WS_OVERLAPPED OR WS_VSCROLL OR WS_HSCROLL
             OR WS_SYSMENU;
    X := ProbBoxX;
    Y := ProbBoxY;
    W := ProbBoxW;
    H := ProbBoxH;
  END;

  (* Set up scroll bars *)
  Scroller := New(PScroller, Init(@self, 10, 10, 14, 94));
END;


DESTRUCTOR TProbWin.Done;
BEGIN
  (* Call ancestor method *)
  TWindow.Done;
END;


PROCEDURE TProbWin.WMVScroll(var Msg: TMessage);
BEGIN
  (* Call ancestor method *)
  TWindow.WMVScroll(Msg);
END;


PROCEDURE TProbWin.Paint(PaintDC: HDC; var PaintInfo: TPaintStruct);
BEGIN
  (* Call ancestor method *)
  TWindow.Paint(PaintDC, PaintInfo);

  (* Call screen update procedure *)
  UpdateScreen(PaintDC);
END;


PROCEDURE TProbWin.UpdateScreen(TheDC: HDC);
VAR
   MadeDC : BOOLEAN;
   TextString: ARRAY [0..5] OF CHAR;
   TextChar: CHAR;
   A, B, C, X, Y: INTEGER;
BEGIN
  (* Check to see if passed a DC *)
  IF (TheDC = 0) THEN
  BEGIN
    TheDC := GetDC(HWindow);
    MadeDC := TRUE;
  END
  ELSE
    MadeDC := FALSE;

  SetBkColor(TheDC, GetSysColor(COLOR_WINDOW));
  SetTextColor(TheDC, GetSysColor(COLOR_WINDOWTEXT));
  SetTextAlign(TheDC, TA_RIGHT);

  (* Actually update the screen *)
  FOR A := 1 TO 4 DO
  BEGIN
    Str(A, TextString);
    TextOut(TheDC, 30+(40*A), 5, TextString, StrLen(TextString));  
  END;

  (* Draw card names *)
  SetTextAlign(TheDC, TA_RIGHT);
  X := 5;
  Y := 15;
  FOR A := 1 TO 4 DO
  BEGIN
    FOR B := 1 TO 13 DO
    BEGIN
      Y := Y + 18;
      TextString[3] := #0;
      TextString[0] := TextCardVals1[B];
      TextString[1] := TextCardVals2[B];
      TextString[2] := TextCardSuits[A];
      TextOut(TheDC, 30, Y, TextString, StrLen(TextString));

      (* Draw actual values *)
      FOR C := 1 TO 4 DO
      BEGIN
        Str(Probability[C, ((A-1)*13+B)], TextString);
        TextOut(TheDC, 30+(40*C), Y, TextString, StrLen(textString));
      END;
    END;
  END;


  (* If we made a DC then release it *)
  IF (MadeDC) THEN
    ReleaseDC(HWindow, TheDC);
END;


FUNCTION TProbWin.CanClose;
BEGIN
  (* Set window size attributes *)
  ProbBoxX := Attr.X;
  ProbBoxY := Attr.Y;
  ProbBoxW := Attr.W;
  ProbBoxH := Attr.H;

  (* Reset parent window fields *)
  PMainWindow(Parent)^.BoxExistProb := FALSE;

  (* Return TRUE as always *)
  CanClose := TRUE;
END;


(************************************************************************)
(* Methods of TNameBox follow... *)

CONSTRUCTOR TNameBox.Init (AParent: PWindowsObject; AName: PChar);
BEGIN
  (* Call ancestor method *)
  TDialog.Init(AParent, AName);

  (* Get handles *)
  NameInput := New(PEdit, InitResource(@self, ID_NameInput, 18));
END;


PROCEDURE TNameBox.OK(VAR Msg: TMessage);
BEGIN
  (* Get text name item *)
  GetDlgItemText(HWindow, ID_NameInput, NameInputText, 18);

  (* Call ancestor method *)
  TDialog.OK(Msg);
END;


(************************************************************************)
(* Methods of TScoreBox follow... *)

CONSTRUCTOR TScoreBox.Init (AParent: PWindowsObject; AName: PChar);
BEGIN

  (* Call ancestor method *)
  TDialog.Init(AParent, AName);

  (* Get handles *)
  Score1  := New(PStatic, InitResource(@self, ID_SCRNAME1, 15));
  Score2  := New(PStatic, InitResource(@self, ID_SCRNAME2, 15));
  Score3  := New(PStatic, InitResource(@self, ID_SCRNAME3, 15));
  Score4  := New(PStatic, InitResource(@self, ID_SCRNAME4, 15));
  Score5  := New(PStatic, InitResource(@self, ID_SCRNAME5, 15));
  Score6  := New(PStatic, InitResource(@self, ID_SCRNAME6, 15));
  Score7  := New(PStatic, InitResource(@self, ID_SCRNAME7, 15));
  Score8  := New(PStatic, InitResource(@self, ID_SCRNAME8, 15));
  Score9  := New(PStatic, InitResource(@self, ID_SCRNAME9, 15));
  Score10 := New(PStatic, InitResource(@self, ID_SCRNAME10, 15));
  Value1  := New(PStatic, InitResource(@self, ID_SCRVALU1, 5));
  Value2  := New(PStatic, InitResource(@self, ID_SCRVALU2, 5));
  Value3  := New(PStatic, InitResource(@self, ID_SCRVALU3, 5));
  Value4  := New(PStatic, InitResource(@self, ID_SCRVALU4, 5));
  Value5  := New(PStatic, InitResource(@self, ID_SCRVALU5, 5));
  Value6  := New(PStatic, InitResource(@self, ID_SCRVALU6, 5));
  Value7  := New(PStatic, InitResource(@self, ID_SCRVALU7, 5));
  Value8  := New(PStatic, InitResource(@self, ID_SCRVALU8, 5));
  Value9  := New(PStatic, InitResource(@self, ID_SCRVALU9, 5));
  Value10 := New(PStatic, InitResource(@self, ID_SCRVALU10, 5));

END;


PROCEDURE TScoreBox.SetupWindow;
VAR
   TextString: ARRAY [0..20] OF CHAR;
BEGIN
  (* Call ancestor method *)
  TDialog.SetupWindow;

  (* Set controls appropriately *)
  Str(BestScores[1], TextString);
  Value1^.SetText(TextString);
  Str(BestScores[2], TextString);
  Value2^.SetText(TextString);
  Str(BestScores[3], TextString);
  Value3^.SetText(TextString);
  Str(BestScores[4], TextString);
  Value4^.SetText(TextString);
  Str(BestScores[5], TextString);
  Value5^.SetText(TextString);
  Str(BestScores[6], TextString);
  Value6^.SetText(TextString);
  Str(BestScores[7], TextString);
  Value7^.SetText(TextString);
  Str(BestScores[8], TextString);
  Value8^.SetText(TextString);
  Str(BestScores[9], TextString);
  Value9^.SetText(TextString);
  Str(BestScores[10], TextString);
  Value10^.SetText(TextString);
  Score1^.SetText(BestNames[1]);
  Score2^.SetText(BestNames[2]);
  Score3^.SetText(BestNames[3]);
  Score4^.SetText(BestNames[4]);
  Score5^.SetText(BestNames[5]);
  Score6^.SetText(BestNames[6]);
  Score7^.SetText(BestNames[7]);
  Score8^.SetText(BestNames[8]);
  Score9^.SetText(BestNames[9]);
  Score10^.SetText(BestNames[10]);
END;


(************************************************************************)
(* Methods of TAboutBox follow... *)


CONSTRUCTOR TAboutBox.Init (AParent: PWindowsObject; AName: PChar);
BEGIN

  (* Call ancesor method *)
  TDialog.Init(AParent, AName);

  (* Get handles to controls *)                              
  Text1 := New(PStatic, InitResource(@self, ID_ABOTTITLE, 15));
  Text2 := New(PStatic, InitResource(@self, ID_COPYRIGHT, 15));
  Text3 := New(PStatic, InitResource(@self, ID_VERSIONNO, 15));
  Text4 := New(PStatic, InitResource(@self, ID_RELSEDATE, 15));
END;


PROCEDURE TAboutBox.SetupWindow;
BEGIN
  (* Call ancestor method *)
  TDialog.SetupWindow;

  (* Set window text *)
  SetWindowText(HWindow, 'About Estimation Whist');
                               
  (* Set controls appropriately *)
  Text1^.SetText('Estimation Whist');
  Text2^.SetText('Copyright © Martin Davidson 1994-2002');
  Text3^.SetText('Version 1.2');
  Text4^.SetText('Released: 01/08/2002');
END;


(************************************************************************)
(* Methods of TChetBox follow... *)


CONSTRUCTOR TChetBox.Init(AParent: PWindowsObject; AName: PChar);
BEGIN

  (* Call ancestor method *)
  TWindow.Init(AParent, AName);

  (* Set up window *)
  WITH Attr DO
  BEGIN
    Style := Style OR WS_POPUP OR WS_CAPTION OR WS_SYSMENU;
    X := PMainWindow(Parent)^.CheatX;
    Y := PMainWindow(Parent)^.CheatY;
    W := 400;
    H := 200;
  END;
END;


DESTRUCTOR TChetBox.Done;
BEGIN
  (* Call ancestor method *)
  TWindow.Done;
END;


PROCEDURE TChetBox.GetWindowClass(VAR AWndClass: TWNDCLASS);
BEGIN
  (* Window particulars *)
  TWindow.GetWindowClass(AWndClass);
  AWndClass.hbrBackGround := COLOR_BTNFACE + 1;
END;


PROCEDURE TChetBox.SetupWindow;
BEGIN
  TWindow.SetupWindow;                        

  (* Call screen drawing routine *)
  DrawScreen(0);
END;


PROCEDURE TChetBox.DrawScreen(TheDC: HDC);
VAR
   MemDC: HDC;
   A, B, C, ActWidth: INTEGER;
   YIncrement: INTEGER;
   MadeDC: BOOLEAN;
   Rectangle: TRect;
   TextString, TextStrtwo: ARRAY [0..30] OF CHAR;
   OldBkColor, OldTextColor: TColorRef;
BEGIN

  (* Check to see if passed a DC *)
  IF (TheDC = 0) THEN
  BEGIN
    (* Otherwise create new DC *)
    TheDC := GetDC(HWindow);
    MadeDC := TRUE;
  END
  ELSE
    MadeDC := FALSE;

  (* Clear the window *)
  WITH Rectangle DO
  BEGIN
    Left := 0;
    Right := Attr.W;
    Top := 0;
    Bottom := Attr.H;
  END;
  FillRect(TheDC, Rectangle, PMainWindow(Parent)^.LtGrayBrush);

  (* Recalculate NoCards to make sure it is OK *)
  NoCards := 0;
  FOR A := 1 TO MaxCards DO
  BEGIN
    IF (HandCards[2, A] > 0) THEN
      INC(NoCards);
  END; 

  (* Draw cards first *)
  MemDC := CreateCompatibleDC(TheDC);

  (* Calculate the Y increment value *)
  YIncrement := 57;
  IF (NoPlayers > 2) THEN YIncrement := ((Attr.H-81) DIV (NoPlayers-2));
  IF (YIncrement > 57) THEN YIncrement := 57;

  FOR A := 2 TO NoPlayers DO
  BEGIN
    (* Draw the player number *)
    OldBkColor := SetBkColor(TheDC, GetSysColor(COLOR_BTNFACE));
    OldTextColor := SetTextColor(TheDC, GetSysColor(COLOR_WINDOWTEXT));
    Str(A, TextString);
    TextOut(TheDC, 10, (14+YIncrement*(A-2)), TextString, Strlen(TextString));
    SetBkColor(TheDC, OldBkColor);
    SetTextColor(TheDC, OldtextColor);

    (* Draw the actual cards themselves *)
    C := 1;
    IF ((NoCards - 1) > 0) THEN
    BEGIN
      ActWidth := (360 - SmallCardWidth) DIV (NoCards - 1);
      IF (ActWidth > SmallMinWidth) THEN
      BEGIN
        IF (ActWidth > (SmallCardWidth + 10)) THEN
          ActWidth := SmallCardWidth + 10;
        FOR B := 1 TO MaxCards DO
        BEGIN
          IF (HandCards[A, B] > 0) THEN
          BEGIN
            SelectObject(MemDC, CardPic[HandCards[A, B]]);
            StretchBlt(TheDC, 30+(C-1)*ActWidth, (4+YIncrement*(A-2)), SmallCardWidth,
                       SmallCardHeight, MemDC, 0, 0, CardWidth, CardHeight,
                       SRCCopy);
            INC(C);
          END;
        END;
      END;
    END
    ELSE
    BEGIN
      FOR B := 1 TO MaxCards DO
      BEGIN
        IF (HandCards[A, B] > 0) THEN
        BEGIN
          SelectObject(MemDC, CardPic[HandCards[A, B]]);
          StretchBlt(TheDC, 30, (4+YIncrement*(A-2)), SmallCardWidth,
                     SmallCardHeight, MemDC, 0, 0, CardWidth, CardHeight,
                     SRCCopy);
        END;
      END;
    END;
  END;

  (* Tidy up *)
  DeleteDC(MemDC);
  IF (MadeDC) THEN ReleaseDC(HWindow, TheDC);
END;


PROCEDURE TChetBox.Paint(PaintDC: HDC; VAR PaintInfo: TPaintStruct);
BEGIN
  (* Call screen drawing routine *)
  DrawScreen(PaintDC);
END;


FUNCTION TChetBox.CanClose: BOOLEAN;
BEGIN
  (* Get window coordinates *)
  PMainWindow(Parent)^.CheatX := Attr.X;
  PMainWindow(Parent)^.CheatY := Attr.Y;

  PMainWindow(Parent)^.CheatCards := FALSE;

  (* Return TRUE as always *)
  CanClose := TRUE;
END;


FUNCTION TChetBox.GetClassName: PChar;
BEGIN
  GetClassName := 'Cheat card window';
END;


(************************************************************************)
(* Methods of TOptions follow... *)


CONSTRUCTOR TOptions.Init(AParent: PWindowsObject; AName: PChar);
BEGIN
  (* Call ancestor method *)
  TDialog.Init (AParent, AName);

  (* Create controls - scroll bars *)
  Txt1 := New(PStatic, InitResource(@self, ID_NUMPLAYS, 3));
  Txt2 := New(PStatic, InitResource(@self, ID_NUMCARDS, 3));
  Scroll1 := New(PScrollBar, InitResource(@self, ID_NUMPLAY));
  Scroll2 := New(PScrollBar, InitResource(@self, ID_NUMCARD));

  (* Create controls - radio buttons... *)
  Rad1 := New(PRadioButton, InitResource(@self, ID_DIALOGRAD));
  Rad2 := New(PRadioButton, InitResource(@self, ID_MOUSERAD));
  Rad3 := New(PRadioButton, InitResource(@self, ID_VANILLA));
  Rad4 := New(PRadioButton, InitResource(@self, ID_SQUARED));

  (* ...and check boxes *)
  Chk1 := New(PCheckBox, InitResource(@self, ID_CHEATCARD));
  Chk2 := New(PCheckBox, InitResource(@self, ID_CONFIRMEX));
  Chk3 := New(PCheckBox, InitResource(@self, ID_SAVEEXIT));
END;


PROCEDURE TOptions.SetupWindow;
VAR
   TextString: ARRAY [0..10] OF CHAR;
BEGIN
  (* Call ancestor method *)
  TDialog.SetUpWindow;

  (* Set up scrollbars *)
  Scroll1^.SetRange(2, MaxPlayNo);
  Scroll1^.PageMagnitude := 1;
  Scroll1^.SetPosition(NoPlayers);
  Scroll2^.SetRange(1, MaxCardNo);
  Scroll2^.PageMagnitude := 1;
  Scroll2^.SetPosition(MaxCards);

  (* Set up static text controls *)
  Str(NoPlayers, TextString);
  Txt1^.SetText(TextString);
  Str(MaxCards, TextString);
  Txt2^.SetText(TextString);

  (* Set controls appropriately *)
  Rad1^.SetCheck(BF_UNCHECKED);
  Rad2^.SetCheck(BF_UNCHECKED);

  IF (PMainWindow(Parent)^.NextCardNotify = Dialog) THEN  Rad1^.SetCheck(BF_CHECKED);
  IF (PMainWindow(Parent)^.NextCardNotify = Mouse)  THEN  Rad2^.SetCheck(BF_CHECKED);

  (* Set up scoring mode playing buttons *)
  Rad3^.SetCheck(BF_UNCHECKED);
  Rad4^.SetCheck(BF_UNCHECKED);

  IF (PMainWindow(Parent)^.ScoreMode = Vanilla) THEN  Rad3^.SetCheck(BF_CHECKED);
  IF (PMainWindow(Parent)^.ScoreMode = Squared) THEN  Rad4^.SetCheck(BF_CHECKED);

  (* Sort out the save on exit and other check boxes *)
  Chk1^.SetCheck(BF_UNCHECKED);
  Chk2^.SetCheck(BF_UNCHECKED);
  Chk3^.SetCheck(BF_UNCHECKED);
  IF (PMainWindow(Parent)^.CheatCards)  THEN Chk1^.SetCheck(BF_CHECKED);
  IF (PMainWindow(Parent)^.ConfirmExit) THEN Chk2^.SetCheck(BF_CHECKED);
  IF (PMainWindow(Parent)^.SaveExit)    THEN Chk3^.SetCheck(BF_CHECKED);
END;


PROCEDURE TOptions.Help(VAR Msg: TMessage);
BEGIN
  (* Call windows help system *)
  WinHelp(HWindow, HELPFILENAME, HELP_CONTEXT, 100);
END;


PROCEDURE TOptions.OK(VAR Msg: TMessage);
BEGIN
  IF (GameInProgress) THEN
  BEGIN
    IF ((Scroll1^.GetPosition <> NoPlayers) OR
        (Scroll2^.GetPosition <> MaxCards)) THEN
    BEGIN
      IF (MessageBox(HWindow, 'Reset values and restart game?',
          'Estimation Whist', (MB_OKCANCEL OR MB_ICONQUESTION)) =
          ID_OK) THEN
      BEGIN
        (* Calling procedure in parent window checks the following value *)
        GameInProgress := FALSE;
      END
      ELSE
        EXIT;
    END;
  END;

  (* Retrieve information from scrollbars *)
  NoPlayers := Scroll1^.GetPosition;
  MaxCards  := Scroll2^.GetPosition;

  (* Get status of dialog item *)
  IF (Rad1^.GetCheck = BF_CHECKED) THEN PMainWindow(Parent)^.NextCardNotify := Dialog;
  IF (Rad2^.GetCheck = BF_CHECKED) THEN PMainWindow(Parent)^.NextCardNotify := Mouse;

  IF (Rad3^.GetCheck = BF_CHECKED) THEN PMainWindow(Parent)^.ScoreMode := Vanilla;
  IF (Rad4^.GetCheck = BF_CHECKED) THEN PMainWindow(Parent)^.ScoreMode := Squared;

  (* Check the save on exit and other check boxes *)
  IF (Chk1^.GetCheck = BF_CHECKED) THEN
    PMainWindow(Parent)^.CheatCards := TRUE
  ELSE
    PMainWindow(Parent)^.CheatCards := FALSE;

  IF (Chk2^.GetCheck = BF_CHECKED) THEN
    PMainWindow(Parent)^.ConfirmExit := TRUE
  ELSE
    PMainWindow(Parent)^.ConfirmExit := FALSE;

  IF (Chk3^.GetCheck = BF_CHECKED) THEN
    PMainWindow(Parent)^.SaveExit := TRUE
  ELSE
    PMainWindow(Parent)^.SaveExit := FALSE;

  (* Call ancestor method *)
  TDialog.OK (Msg);
END;


PROCEDURE TOptions.HanNumPlaySc(VAR Msg: TMessage);
VAR
   TextString: ARRAY [0..10] OF CHAR;
   Maximum, Pos: INTEGER;
BEGIN
  (* Check scrollbar position is valid *)
  IF ((52 DIV Scroll1^.GetPosition) < MaxCardNo) THEN
    Maximum := (52 DIV Scroll1^.GetPosition)
  ELSE
    Maximum := MaxCardNo;

  IF (Scroll2^.GetPosition > Maximum) THEN
  BEGIN
    Scroll2^.SetPosition(Maximum);
    Str(Maximum, TextString);
    Txt2^.SetText(TextString);
  END;

  (* Update static text control *)
  Str(Scroll1^.GetPosition, TextString);
  Txt1^.SetText(TextString);
END;


PROCEDURE TOptions.HanNumCardSc(VAR Msg: TMessage);
VAR
   TextString: ARRAY [0..10] OF CHAR;
   Maximum, Pos: INTEGER;
BEGIN
  (* Check scrollbar position is valid *)
  IF ((52 DIV Scroll1^.GetPosition) < MaxCardNo) THEN
    Maximum := (52 DIV Scroll1^.GetPosition)
  ELSE
    Maximum := MaxCardNo;

  IF (Scroll2^.GetPosition > Maximum) THEN
    Pos := Maximum
  ELSE
    Pos := Scroll2^.GetPosition;

  Scroll2^.SetPosition(Pos);

  (* Update static text control *)
  Str(Scroll2^.GetPosition, TextString);
  Txt2^.SetText(TextString);
END; 


(************************************************************************)
(* Methods of TRandom follow... *)


CONSTRUCTOR TRandom.Init(AParent: PWindowsObject; AName: PChar);
BEGIN
  (* Call ancestor method *)
  TDialog.Init (AParent, AName);

  (* Create controls *)
  RndMultSt := New(PStatic, InitResource(@self, ID_RndMultSt, 3));
  RndNumbSt := New(PStatic, InitResource(@self, ID_RndNumbSt, 3));
  RndTimeSt := New(PStatic, InitResource(@self, ID_RndTimeSt, 3));
  RndMultSc := New(PScrollBar, InitResource(@self, ID_RndMultSc));
  RndNumbSc := New(PScrollBar, InitResource(@self, ID_RndNumbSc));
  RndTimeSc := New(PScrollBar, InitResource(@self, ID_RndTimeSc));
  RndExisCk := New(PCheckBox, InitResource(@self, ID_RndExisCk));
  RndIconCk := New(PCheckBox, InitResource(@self, ID_RndIconCk));
END;


PROCEDURE TRandom.SetupWindow;
VAR
   TextString: ARRAY [0..10] OF CHAR;
BEGIN
  (* Call ancestor method *)
  TDialog.SetUpWindow;

  (* Set up scrollbars *)
  RndMultSc^.SetRange(1, 20);
  RndMultSc^.PageMagnitude := 1;
  RndMultSc^.SetPosition(Multiplier);
  RndNumbSc^.SetRange(1, 6);
  RndNumbSc^.PageMagnitude := 1;
  RndNumbSc^.SetPosition(NoThings);
  RndTimeSc^.SetRange(20, 1000);
  RndTimeSc^.PageMagnitude := 100;
  RndTimeSc^.SetPosition(TimeRnd);

  (* Set up static text controls *)
  Str(Multiplier, TextString);
  RndMultSt^.SetText(TextString);
  Str(NoThings, TextString);
  RndNumbSt^.SetText(TextString);
  Str(TimeRnd, TextString);
  RndTimeSt^.SetText(TextString);

  (* Set up check boxes *)
  IF (ExistRnd) THEN
    RndExisCk^.SetCheck(BF_CHECKED)
  ELSE
    RndExisCk^.SetCheck(BF_UNCHECKED);

  IF (ExistIcn) THEN
    RndIconCk^.SetCheck(BF_CHECKED)
  ELSE
    RndIconCk^.SetCheck(BF_UNCHECKED);
END;


PROCEDURE TRandom.Help(VAR Msg: TMessage);
BEGIN
  (* Call windows help system *)
  WinHelp(HWindow, HELPFILENAME, HELP_CONTEXT, 101);
END;


PROCEDURE TRandom.OK(VAR Msg: TMessage);
BEGIN
  (* Retrieve information from scrollbars *)
  Multiplier := RndMultSc^.GetPosition;
  NoThings := RndNumbSc^.GetPosition;
  TimeRnd := RndTimeSc^.GetPosition;

  (* Retrieve check box information *)
  IF (RndExisCk^.GetCheck = BF_CHECKED) THEN
    ExistRnd := TRUE
  ELSE
    ExistRnd := FALSE;

  IF (RndIconCk^.GetCheck = BF_CHECKED) THEN
    ExistIcn := TRUE
  ELSE
    ExistIcn := FALSE;

  (* Call ancestor method *)
  TDialog.OK (Msg);
END;


PROCEDURE TRandom.HanRndMultSc (VAR Msg: TMessage);
VAR
   TextString: ARRAY [0..10] OF CHAR;
BEGIN
  (* Update static text control *)
  Str(RndMultSc^.GetPosition, TextString);
  RndMultSt^.SetText(TextString);
END;


PROCEDURE TRandom.HanRndNumbSc (VAR Msg: TMessage);
VAR
   TextString: ARRAY [0..10] OF CHAR;
BEGIN
  (* Update static text control *)
  Str(RndNumbSc^.GetPosition, TextString);
  RndNumbSt^.SetText(TextString);
END;


PROCEDURE TRandom.HanRndTimeSc (VAR Msg: TMessage);
VAR
   TextString: ARRAY [0..10] OF CHAR;
BEGIN
  (* Update static text control *)
  Str(RndTimeSc^.GetPosition, TextString);
  RndTimeSt^.SetText(TextString);
END;



(************************************************************************)
(* Methods of TCallWin follow... *)


CONSTRUCTOR TCallWin.Init (AParent: PWindowsObject; AName: PChar);
BEGIN
  (* Call ancestor method *)
  TDialog.Init (AParent, AName);

  (* Set up dialog button objects *)
  CallBut0 := New(PButton, InitResource(@self, ID_CALLZER));
  CallBut1 := New(PButton, InitResource(@self, ID_CALLONE));
  CallBut2 := New(PButton, InitResource(@self, ID_CALLTWO));
  CallBut3 := New(PButton, InitResource(@self, ID_CALLTHR));
  CallBut4 := New(PButton, InitResource(@self, ID_CALLFOU));
  CallBut5 := New(PButton, InitResource(@self, ID_CALLFIV));
  CallBut6 := New(PButton, InitResource(@self, ID_CALLSIX));
  CallBut7 := New(PButton, InitResource(@self, ID_CALLSEV));
  CallBut8 := New(PButton, InitResource(@self, ID_CALLEIG));
  CallBut9 := New(PButton, InitResource(@self, ID_CALLNIN));
  CallBut10 := New(PButton, InitResource(@self, ID_CALLTEN));
  CallBut11 := New(PButton, InitResource(@self, ID_CALLELE));
  CallBut12 := New(PButton, InitResource(@self, ID_CALLTWE));
  CallBut13 := New(PButton, InitResource(@self, ID_CALLTHT));
  CallBut14 := New(PButton, InitResource(@self, ID_CALLFOT));
  CallBut15 := New(PButton, InitResource(@self, ID_CALLFIF));
END;


PROCEDURE TCallWin.SetUpWindow;
VAR
   A, B: INTEGER;
   TextString, TextStrtwo: ARRAY [0..10] OF CHAR;
BEGIN
  (* Call ancestor method *)
  TDialog.SetUpWindow;

  (* Check to see what are valid calls *)
  A := NoCards;
  IF (A < 1) THEN EnableWindow(CallBut1^.HWindow, FALSE);
  IF (A < 2) THEN EnableWindow(CallBut2^.HWindow, FALSE);
  IF (A < 3) THEN EnableWindow(CallBut3^.HWindow, FALSE);
  IF (A < 4) THEN EnableWindow(CallBut4^.HWindow, FALSE);
  IF (A < 5) THEN EnableWindow(CallBut5^.HWindow, FALSE);
  IF (A < 6) THEN EnableWindow(CallBut6^.HWindow, FALSE);
  IF (A < 7) THEN EnableWindow(CallBut7^.HWindow, FALSE);
  IF (A < 8) THEN EnableWindow(CallBut8^.HWindow, FALSE);
  IF (A < 9) THEN EnableWindow(CallBut9^.HWindow, FALSE);
  IF (A < 10) THEN EnableWindow(CallBut10^.HWindow, FALSE);
  IF (A < 11) THEN EnableWindow(CallBut11^.HWindow, FALSE);
  IF (A < 12) THEN EnableWindow(CallBut12^.HWindow, FALSE);
  IF (A < 13) THEN EnableWindow(CallBut13^.HWindow, FALSE);
  IF (A < 14) THEN EnableWindow(CallBut14^.HWindow, FALSE);
  IF (A < 15) THEN EnableWindow(CallBut15^.HWindow, FALSE);

  (* If playing last then he cannot call a particular number of tricks *)
  IF (StartPlayer = 2) THEN
  BEGIN
    B := 0;
    FOR A := 2 TO NoPlayers DO
      B := B + PlayCall[A];
    B := NoCards - B;
  END;

  CASE B OF
     0 : EnableWindow(CallBut0^.HWindow, FALSE);
     1 : EnableWindow(CallBut1^.HWindow, FALSE);
     2 : EnableWindow(CallBut2^.HWindow, FALSE); 
     3 : EnableWindow(CallBut3^.HWindow, FALSE);
     4 : EnableWindow(CallBut4^.HWindow, FALSE);
     5 : EnableWindow(CallBut5^.HWindow, FALSE); 
     6 : EnableWindow(CallBut6^.HWindow, FALSE);
     7 : EnableWindow(CallBut7^.HWindow, FALSE);
     8 : EnableWindow(CallBut8^.HWindow, FALSE); 
     9 : EnableWindow(CallBut9^.HWindow, FALSE);
    10 : EnableWindow(CallBut10^.HWindow, FALSE);
    11 : EnableWindow(CallBut11^.HWindow, FALSE); 
    12 : EnableWindow(CallBut12^.HWindow, FALSE);
    13 : EnableWindow(CallBut13^.HWindow, FALSE);
    14 : EnableWindow(CallBut14^.HWindow, FALSE); 
    15 : EnableWindow(CallBut15^.HWindow, FALSE);
  END;
END;


PROCEDURE TCallWin.CallZer (VAR Msg: TMessage);
BEGIN
  (* Player has called zero tricks *)
  PlayCall[1] := 0;

  (* Call dialog end mehod *)
  TDialog.OK(Msg);
END;


PROCEDURE TCallWin.CallOne (VAR Msg: TMessage);
BEGIN
  (* Player has called one trick *)
  PlayCall[1] := 1;

  (* Call dialog end mehod *)
  TDialog.OK(Msg);
END;


PROCEDURE TCallWin.CallTwo (VAR Msg: TMessage);
BEGIN
  (* Player has called two tricks *)
  PlayCall[1] := 2;

  (* Call dialog end mehod *)
  TDialog.OK(Msg);
END;


PROCEDURE TCallWin.CallThr (VAR Msg: TMessage);
BEGIN
  (* Player has called three tricks *)
  PlayCall[1] := 3;

  (* Call dialog end mehod *)
  TDialog.OK(Msg);
END;


PROCEDURE TCallWin.CallFou (VAR Msg: TMessage);
BEGIN
  (* Player has called four tricks *)
  PlayCall[1] := 4;

  (* Call dialog end mehod *)
  TDialog.OK(Msg);
END;


PROCEDURE TCallWin.CallFiv (VAR Msg: TMessage);
BEGIN
  (* Player has called five tricks *)
  PlayCall[1] := 5;

  (* Call dialog end mehod *)
  TDialog.OK(Msg);
END;


PROCEDURE TCallWin.CallSix (VAR Msg: TMessage);
BEGIN
  (* Player has called six tricks *)
  PlayCall[1] := 6;

  (* Call dialog end mehod *)
  TDialog.OK(Msg);
END;


PROCEDURE TCallWin.CallSev (VAR Msg: TMessage);
BEGIN
  (* Player has called seven tricks *)
  PlayCall[1] := 7;

  (* Call dialog end mehod *)
  TDialog.OK(Msg);
END;


PROCEDURE TCallWin.CallEig (VAR Msg: TMessage);
BEGIN
  (* Player has called eight tricks *)
  PlayCall[1] := 8;

  (* Call dialog end mehod *)
  TDialog.OK(Msg);
END;


PROCEDURE TCallWin.CallNin (VAR Msg: TMessage);
BEGIN
  (* Player has called nine tricks *)
  PlayCall[1] := 9;

  (* Call dialog end mehod *)
  TDialog.OK(Msg);
END;


PROCEDURE TCallWin.CallTen (VAR Msg: TMessage);
BEGIN
  (* Player has called ten tricks *)
  PlayCall[1] := 10;

  (* Call dialog end mehod *)
  TDialog.OK(Msg);
END;


PROCEDURE TCallWin.CallEle (VAR Msg: TMessage);
BEGIN
  (* Player has called eleven tricks *)
  PlayCall[1] := 11;

  (* Call dialog end mehod *)
  TDialog.OK(Msg);
END;


PROCEDURE TCallWin.CallTwe (VAR Msg: TMessage);
BEGIN
  (* Player has called twelve tricks *)
  PlayCall[1] := 12;

  (* Call dialog end mehod *)
  TDialog.OK(Msg);
END;


PROCEDURE TCallWin.CallTht (VAR Msg: TMessage);
BEGIN
  (* Player has called thirteen tricks *)
  PlayCall[1] := 13;

  (* Call dialog end mehod *)
  TDialog.OK(Msg);
END;


PROCEDURE TCallWin.CallFot (VAR Msg: TMessage);
BEGIN
  (* Player has called fourteen tricks *)
  PlayCall[1] := 14;

  (* Call dialog end mehod *)
  TDialog.OK(Msg);
END;


PROCEDURE TCallWin.CallFif (VAR Msg: TMessage);
BEGIN
  (* Player has called fifteen tricks *)
  PlayCall[1] := 15;

  (* Call dialog end mehod *)
  TDialog.OK(Msg);
END;


(************************************************************************)
(* Main program follows *)

VAR
   CardApp: TCardApp;

BEGIN
  (* Load custom controls *)
  Ctl3dRegister(HInstance);
  Ctl3dAutoSubclass(HInstance);

  CardApp.Init('ESTWHI');
  CardApp.Run;
  CardApp.Done;

  (* Unload custom controls *)
  Ctl3dUnRegister(HInstance);
END.