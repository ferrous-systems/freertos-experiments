// A basic stdint.h header to make FreeRTOS happy
//
// Should match the C library your C compiler (probably gcc-arm-none-eabi) uses.
//
// SPDX-FileCopyrightText: Copyright (c) 2024 Ferrous Systems
// SPDX-License-Identifier: CC0-1.0

#ifndef _STDINC_H
#define _STDINC_H

typedef unsigned long uint32_t;
typedef signed long int32_t;
typedef unsigned long size_t;
typedef unsigned char uint8_t;
typedef unsigned short uint16_t;

#endif