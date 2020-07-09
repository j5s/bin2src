program PascalExample;

{$mode objfpc}

uses
Windows, MMsystem, fpWavFormat, Tport;

const
  (* System messages *)
  MsgEAllocated	= 'Specified resource is already allocated.';
  MsgEBadDev = 'Specified device identifier is out of range.';
  MsgENoDev = 'No device driver is present.';
  MsgENoMem = 'Unable to allocate or lock memory.';
  MsgEBadFmt = 'Attempted to open with an unsupported waveform-audio format.';
  MsgESync = 'The device is synchronous but waveOutOpen was called without using the WAVE_ALLOWSYNC flag.';
  MsgEInvHnd = 'Specified device handle is invalid.';
  MsgEUnPrep = 'The data block pointed to by the pwh parameter hasn''t been prepared.';
  MsgESPUnPrep = 'The data block pointed to by the pwh parameter is still in the queue.';
  MsgESPClose = 'There are still buffers in the queue.';

  (* Offsets *)
  WFmtOffset = SizeOf(TRiffHeader);
  WDataHeaderOffset = WFmtOffset + SizeOf(TWaveFormat);
  WDataOffset = WDataHeaderOffset + SizeOf(TChunkHeader);

type
  (* Enum of operations, used for output error messages *)
  Op = (
	opOpenDev,
	opPrepareHeader,
	opWriteDev,
	opUnprepareHeader,
	opCloseDev
	);
  
  (* Pointer types *)
  PWaveFormat = ^TWaveFormat;
  PWaveDataHeader = ^TChunkHeader;

(* Write error message and return true if an error ocurred *)
function HandleErr(operation : Op; res: MMRESULT): Boolean;
begin
  if res = MMSYSERR_NOERROR then
    exit(false)
  else
    result := true;

  case operation of
    opOpenDev :
      begin
        case res of
          MMSYSERR_ALLOCATED: WriteLn(MsgEAllocated);
          MMSYSERR_BADDEVICEID: WriteLn(MsgEBadDev);
          MMSYSERR_NODRIVER: WriteLn(MsgENoDev);
          MMSYSERR_NOMEM: WriteLn(MsgENoMem);
          WAVERR_BADFORMAT: WriteLn(MsgEBadFmt);
          WAVERR_SYNC: WriteLn(MsgESync);
        end;
      end;
    opPrepareHeader :
      begin
        case res of
          MMSYSERR_INVALHANDLE: WriteLn(MsgEInvHnd);
          MMSYSERR_NODRIVER: WriteLn(MsgENoDev);
          MMSYSERR_NOMEM: WriteLn(MsgENoMem);
        end;
      end;
    opWriteDev :
      begin
        case res of
          MMSYSERR_INVALHANDLE: WriteLn(MsgEInvHnd);
          MMSYSERR_NODRIVER: WriteLn(MsgENoDev);
          MMSYSERR_NOMEM: WriteLn(MsgENoMem);
          WAVERR_UNPREPARED: WriteLn(MsgEUnPrep);
        end;
      end;
    opUnprepareHeader :
      begin
        case res of
          MMSYSERR_INVALHANDLE: WriteLn(MsgEInvHnd);
          MMSYSERR_NODRIVER: WriteLn(MsgENoDev);
          MMSYSERR_NOMEM: WriteLn(MsgENoMem);
          WAVERR_STILLPLAYING: WriteLn(MsgESPUnPrep);
        end;
      end;
    opCloseDev :
      begin
        case res of
          MMSYSERR_INVALHANDLE: WriteLn(MsgEInvHnd);
          MMSYSERR_NODRIVER: WriteLn(MsgENoDev);
          MMSYSERR_NOMEM: WriteLn(MsgENoMem);
          WAVERR_STILLPLAYING: WriteLn(MsgESPClose);
        end;
      end;
  end;
end;

(* Safe copy wave struct *)
procedure CopyWaveStruct(i: PWaveFormat; var o: WAVEFORMATEX);
begin
   with i^ do
   begin
      o.wFormatTag := Format;
      o.nChannels := Channels;
      o.nSamplesPerSec := SampleRate;
      o.nAvgBytesPerSec := ByteRate;
      o.nBlockAlign := BlockAlign;
      o.wBitsPerSample := BitsPerSample;
      o.cbSize := 0;
   end;
end;

(* Wave out callback *)
procedure WaveOutCallBack(
  hwo: HWAVEOUT;
  uMsg: UINT;
  dwInstance: DWORD_PTR;
  dwParam1: DWORD_PTR;
  dwParam2: DWORD_PTR
); stdcall;
begin
  case uMsg of
    WOM_OPEN: WriteLn('Device opened');
    WOM_CLOSE: WriteLn('Device closed');
    WOM_DONE:
      begin
	WriteLn('Device done');
	RTLEventSetEvent(PRTLEvent(dwInstance));
      end;
  end;
end;

var
  EvtEnd: PRTLEvent; // Event handler

  (* WAV format *)
  wavefmt: PWaveFormat;
  wavefmtex: WAVEFORMATEX;
  (* WAV data *)
  wavedataheader: PWaveDataHeader;
  wavehdr: TWAVEHDR;
  data: PChar;
  dev: HWAVEOUT;
  res: MMRESULT;

begin
   (* Get number of devices *)
  if waveOutGetNumDevs() > 0 then
  begin
    (* Create an event to handle the end of play *)
    EvtEnd := RTLEventCreate;

    (* Setup pointers to data *)
    wavefmt := PWaveFormat(PByte(@Tport_data) + WFmtOffset);
    wavedataheader := PWaveDataHeader(PByte(@Tport_data) + WDataHeaderOffset);
    data := PChar(PByte(@Tport_data) + WDataOffset);
    CopyWaveStruct(wavefmt, wavefmtex);

    (* Open output device for play *)
    res := waveOutOpen(
		       @dev,
		       0,
		       @wavefmtex,
		       QWord(@WaveOutCallBack),
		       QWord(EvtEnd),
		       CALLBACK_FUNCTION or
		       WAVE_ALLOWSYNC or
		       WAVE_FORMAT_DIRECT
		       );
    if HandleErr(opOpenDev, res) then
      exit;

    (* Prepare buffer *)
    with wavehdr do
    begin
      lpData:= data;
      dwBufferLength := wavedataheader^.Size;
      dwBytesRecorded := 0;
      dwUser := 0;
      dwFlags := WHDR_BEGINLOOP or WHDR_ENDLOOP;
      dwLoops := 1;
      lpNext := Nil;
      reserved := 0;
    end;
    res := waveOutPrepareHeader(dev, @wavehdr, SizeOf(TWAVEHDR));
    if HandleErr(opPrepareHeader, res) then
      exit;

    (* Start playing the buffer data *)
    res := waveOutWrite(dev, @wavehdr, SizeOf(TWAVEHDR));
    if HandleErr(opWriteDev, res) then
      exit;

    (* Wait for the event signaling the end of data *)
    RTLEventWaitFor(EvtEnd);

    (* Free buffer *)
    res := waveOutUnprepareHeader(dev, @wavehdr, SizeOf(TWAVEHDR));
    if HandleErr(opUnprepareHeader, res) then
      exit;

    (* Close output device *)
    res := waveOutClose(dev);
    if HandleErr(opCloseDev, res) then
      exit;

    (* Destroy event *)
    RTLEventDestroy(EvtEnd);
  end else
    writeln('No audio output devices found');
end.
