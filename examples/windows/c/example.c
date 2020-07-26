/*
Embed an image inside code

Binary data of the image is at the file: smbmp.c

 */
#include <windows.h>
#include <stdio.h>
#include "smbmp.h"

const CHAR AUTHOR[] = "Alexandre Gomiero de Oliveira";
const CHAR REPO[] = "https://github.com/gomiero/bin2src";

// Window class declarations
const CHAR CLASS_NAME[] = "bin2src C example";
WNDCLASSEXA winClass;

// Image handle
HBITMAP photo;

// Function prototypes
int WINAPI WinMain(HINSTANCE hInst, HINSTANCE hPrevInst, LPSTR pCmdLine, int nCmdShow);
ATOM RegClass(WNDCLASSEXA* wc, HINSTANCE hInst);
int UnRegClass(HINSTANCE hInst);
LRESULT WINAPI wndProc(HWND hWnd, UINT msg, WPARAM wParam, LPARAM lParam);

// Main
int CALLBACK WinMain(HINSTANCE hInst, HINSTANCE hPrevInst, LPSTR pCmdLine, int nCmdShow)
{
  HWND hWnd;
  MSG msg;

  UNREFERENCED_PARAMETER(hPrevInst);
  UNREFERENCED_PARAMETER(pCmdLine);

  RegClass(&winClass, hInst);
  hWnd = CreateWindowExA(
    0,
    CLASS_NAME,
    "bin2src Test",
    WS_OVERLAPPEDWINDOW,
    600, 300,
    443, 679,
    NULL,
    NULL,
    hInst,
    NULL
  );
  if (hWnd == NULL) 
  {
    fprintf(stderr, "Create window error.\n");
    return -1;
  }
  ShowWindow(hWnd, nCmdShow);
  UpdateWindow(hWnd);
  while (GetMessageA(&msg, NULL, 0, 0))
  {
    TranslateMessage(&msg);
    DispatchMessageA(&msg);
  }
  UnRegClass(hInst);
  return S_OK;
}

// Register class
ATOM RegClass(WNDCLASSEXA *wc, HINSTANCE hInst)
{
  wc->cbSize = sizeof(WNDCLASSEXA);
  wc->style = CS_HREDRAW | CS_VREDRAW;
  wc->lpfnWndProc = &wndProc;
  wc->hInstance = hInst;
  wc->hCursor = LoadCursor(NULL, IDC_ARROW);
  wc->lpszClassName = CLASS_NAME;
  return RegisterClassExA(&winClass);
}

// Unregister class
BOOL UnRegClass(HINSTANCE hInst)
{
  return UnregisterClassA(CLASS_NAME, hInst);
}

// Main window procedure
LRESULT CALLBACK wndProc(HWND hWnd, UINT msg, WPARAM wParam, LPARAM lParam)
{
  switch (msg)
  {
  case WM_CREATE:
  {
    // Message WM_CREATE analize the header of the bitmap file
    // embeded and allocate a HBITMAP
    LPBITMAPFILEHEADER fh;
    PBITMAPINFOHEADER ih;
    VOID* imgdata;
    BITMAPINFO bi;

    fh = (LPBITMAPFILEHEADER)&smbmp_data[0];
    ih = (PBITMAPINFOHEADER)&smbmp_data[0 + sizeof(BITMAPFILEHEADER)];
    imgdata = (VOID *)&smbmp_data[fh->bfOffBits];
    memcpy(&bi.bmiHeader, ih, sizeof(BITMAPINFOHEADER));
    bi.bmiColors->rgbRed = 255;
    bi.bmiColors->rgbGreen = 255;
    bi.bmiColors->rgbBlue = 255;
    bi.bmiColors->rgbReserved = 0;
    HDC hdc = GetDC(hWnd);
    photo = CreateCompatibleBitmap(hdc, 427, 640);
    if (!photo)
    {
      DWORD err = GetLastError();
      fprintf(stderr, "Error: %lx\n", err);
    };
    int res = SetDIBits(hdc, photo, 0, 640, imgdata, &bi, DIB_RGB_COLORS);    
    if (!res)
    {
      DWORD err = GetLastError();
      fprintf(stderr, "Error: %lx\n", err);
    };
    ReleaseDC(hWnd, hdc);
  };
  break;

  case WM_PAINT:
  {
    // On WM_PAINT, select the handle stored at photo
    // and copy it to screen (BitBlt)
    PAINTSTRUCT ps;
    HDC hdc;
    HDC hdcMemory;
    BOOL res;

    hdc = BeginPaint(hWnd, &ps);
    hdcMemory = CreateCompatibleDC(hdc);
    SelectObject(hdcMemory, photo);
    res = BitBlt(hdc, 0, 0, 427, 640, hdcMemory, 0, 0, SRCCOPY);
    if (!res)
    {
      DWORD err = GetLastError();
      fprintf(stderr, "Copy to GDI error: %lx\n", err);
    }
    DeleteDC(hdcMemory);
    EndPaint(hWnd, &ps);
  };
  break;

  case WM_CLOSE:
    DestroyWindow(hWnd);
    return 0;

  case WM_DESTROY:
    DeleteObject(photo);
    PostQuitMessage(0);
    return 0;
  }
  return DefWindowProcA(hWnd, msg, wParam, lParam);
}
