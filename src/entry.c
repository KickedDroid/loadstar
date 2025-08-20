#include "beacon.h"
#include <windows.h>

extern void rust_bof(char* args, int alen);

void go(char* args, int alen) {
    rust_bof(args, alen);
}


extern HANDLE starLoadLibraryA(PCSTR libname) {
    LoadLibraryA(libname);
}

extern FARPROC starGetProcAddress(HANDLE hmodule, PCSTR lpprocname){
    GetProcAddress(hmodule, lpprocname);
}



#ifdef UTILS
extern void starOutput(int type, char* args, int alen) {
    BeaconOutput(0, args, alen);
}


extern void starPrintf(int type, const char *fmt, const char *arg) {
    BeaconPrintf(type, fmt, arg);
}


extern BOOL starToWideChar(char* src,wchar_t * dst, int len) {
    toWideChar(src, dst, len);
}
#endif



#ifdef BEACON_API

extern void starFormatAlloc(formatp * format, int maxsz) {
    BeaconFormatAlloc(format, maxsz);
}

extern void starFormatFree(formatp * format) {
    BeaconFormatFree(format);
}

extern void starDataParse(datap * parser, char* args, int alen) {
    BeaconDataParse(parser, args, alen);
}

extern char* starDataExtract(datap * parser, int size) {
    char* bytes = BeaconDataExtract(parser, 0);
    return bytes;
}

extern int starDataInt(datap * parser) {
    BeaconDataInt(parser);
}

extern char* starFormatToString(formatp * obj, int * size) {
    BeaconFormatToString(obj, size);
}

extern void starFormatAppend(formatp * format, const char * text, int len) {
    BeaconFormatAppend(format, text, len);
}

extern void startFormatReset(formatp * format) {
    BeaconFormatReset(format);
}
#endif
/*extern int starVirtualAlloc(LPVOID lpAddress, SIZE_T dwSize, DWORD flAllocationType, DWORD flProtect) {
    BeaconVirtualAlloc(lpAddress, dwSize, flAllocationType, flProtect);
}*/
