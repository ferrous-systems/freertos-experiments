// SPDX-FileCopyrightText: Copyright (c) 2024 Ferrous Systems
// SPDX-License-Identifier: CC0-1.0

#include <FreeRTOS.h>
#include <portmacro.h>
#include <queue.h>
#include <semphr.h>
#include <task.h>
#include <timers.h>

// bindgen can't see these macros for some reason, so promote them to const
// static for the purposes of code generation.
static const uint32_t _portTICK_PERIOD_MS = portTICK_PERIOD_MS;
static const uint8_t _queueQUEUE_TYPE_BASE = queueQUEUE_TYPE_BASE;
static const int32_t _queueSEND_TO_BACK = queueSEND_TO_BACK;
static const TickType_t _portMAX_DELAY = portMAX_DELAY;
