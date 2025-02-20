import { useEffect, useState, useRef } from "react";

interface TimerProps {
  /** For showing "time worked" */
  startTime: number;
  /** Showing "time left" */
  endTime: number;
  className?: string;
  /** Directly edited the timer and this is the difference between the new time and the original time */
  onAdjustTime?: (ms: number) => void;
}

const TIME_PATTERNS = [
  // 1:30, 1:00, etc
  {
    regex: /^(\d+):(\d{1,2})$/,
    parse: (matches: RegExpMatchArray) => {
      const minutes = parseInt(matches[1], 10);
      const seconds = parseInt(matches[2], 10);
      return { ms: (minutes * 60 + seconds) * 1000 };
    },
  },
  // 90s, 90sec, 90seconds
  {
    regex: /^(\d+)\s*(s|sec|seconds?)$/i,
    parse: (matches: RegExpMatchArray) => ({ ms: parseInt(matches[1], 10) * 1000 }),
  },
  // 5m, 5min, 5minutes
  {
    regex: /^(\d+)\s*(m|min|minutes?)$/i,
    parse: (matches: RegExpMatchArray) => ({ ms: parseInt(matches[1], 10) * 60 * 1000 }),
  },
  // 1h, 1hr, 1hour, 1hours
  {
    regex: /^(\d+)\s*(h|hr|hours?)$/i,
    parse: (matches: RegExpMatchArray) => ({ ms: parseInt(matches[1], 10) * 60 * 60 * 1000 }),
  },
  // Plain numbers are interpreted as minutes
  {
    regex: /^(\d+)$/,
    parse: (matches: RegExpMatchArray) => ({ ms: parseInt(matches[1], 10) * 60 * 1000 }),
  },
];

function parseTimeInput(input: string): { ms: number } | null {
  input = input.trim();

  for (const pattern of TIME_PATTERNS) {
    const matches = input.match(pattern.regex);
    if (matches) {
      try {
        return pattern.parse(matches);
      } catch (e) {
        console.warn("Failed to parse time input:", e);
        return null;
      }
    }
  }

  return null;
}

export function Timer({ startTime, endTime, className, onAdjustTime }: TimerProps) {
  const [time, setTime] = useState(0);
  const [isEditing, setIsEditing] = useState(false);
  const [inputError, setInputError] = useState(false);
  const [isCountingDown, setIsCountingDown] = useState(true);
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    // Reset elapsed when startTime changes
    setTime(Date.now());

    const interval = setInterval(() => {
      if (!isEditing) {
        setTime(Date.now());
      }
    }, 500);

    return () => clearInterval(interval);
  }, [isEditing]);

  const timeInMs = isCountingDown ? endTime - time : time - startTime;

  const minutes = Math.floor(timeInMs / (1000 * 60));
  const seconds = Math.abs(Math.floor((timeInMs / 1000) % 60));

  const handleClick = () => {
    setIsCountingDown((a) => !a);
  };

  const handleDoubleClick = () => {
    if (onAdjustTime) {
      handleStartEdit();
    }
  };

  const handleStartEdit = () => {
    if (!onAdjustTime) return;
    setIsEditing(true);
    setInputError(false);
    setTimeout(() => inputRef.current?.select(), 0);
  };

  const handleFinishEdit = () => {
    if (!inputRef.current || !onAdjustTime) return;

    const newTimeMs = parseTimeInput(inputRef.current.value);

    if (newTimeMs === null) {
      setInputError(true);
      return;
    }
    onAdjustTime(newTimeMs.ms);
    setIsEditing(false);
    setInputError(false);
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "Enter") {
      handleFinishEdit();
    } else if (e.key === "Escape") {
      setIsEditing(false);
      setInputError(false);
    }
  };

  if (isEditing) {
    return (
      <input
        ref={inputRef}
        type="text"
        className={`max-w-16 text-start rounded bg-white border field-sizing-content p-0 m-0
          ${inputError ? "border-red-500" : "border-gray-300"} 
          ${className}`}
        defaultValue={`${minutes}:${seconds.toString().padStart(2, "0")}`}
        placeholder="1:30, 5m..."
        onBlur={handleFinishEdit}
        onKeyDown={handleKeyDown}
        onChange={() => inputError && setInputError(false)}
      />
    );
  }

  return (
    <span
      className={`${className} cursor-pointer hover:text-blue-600`}
      onClick={handleClick}
      onDoubleClick={handleDoubleClick}
      title={
        `${startTime}->${endTime}|` +
        (onAdjustTime
          ? "Double-click to edit (examples: 5m, 1:30, 90s). Click to toggle count mode."
          : "Click to toggle between time elapsed and time remaining")
      }
    >
      {`${timeInMs < 0 ? "-" : ""}${Math.abs(minutes)}:${seconds.toString().padStart(2, "0")}`}
    </span>
  );
}
