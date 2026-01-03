package ui

import (
	lipgloss "github.com/charmbracelet/lipgloss"
)

var (
	primaryColor   = lipgloss.Color("#7C3AED") // Purple
	secondaryColor = lipgloss.Color("#10B981") // Green
	accentColor    = lipgloss.Color("#F59E0B") // Orange
	textColor      = lipgloss.Color("#E5E7EB") // Light gray
	mutedColor     = lipgloss.Color("#9CA3AF") // Muted gray
	errorColor     = lipgloss.Color("#EF4444") // Red
	bgColor        = lipgloss.Color("#1F2937") // Dark gray
)

func PrimaryColor() lipgloss.Color {
	return primaryColor
}

func SecondaryColor() lipgloss.Color {
	return secondaryColor
}

func AccentColor() lipgloss.Color {
	return accentColor
}

func TextColor() lipgloss.Color {
	return textColor
}

func MutedColor() lipgloss.Color {
	return mutedColor
}

func ErrorColor() lipgloss.Color {
	return errorColor
}

func BgColor() lipgloss.Color {
	return bgColor
}
