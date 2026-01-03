package ui

import (
	"github.com/charmbracelet/lipgloss"
)

var (
	HeaderStyle = lipgloss.NewStyle().
			Foreground(primaryColor).
			Bold(true).
			Padding(0, 1).
			MarginBottom(1)

	TodoItemStyle = lipgloss.NewStyle().
			Foreground(textColor).
			Padding(0, 1)

	SelectedItemStyle = lipgloss.NewStyle().
				Foreground(primaryColor).
				Bold(true).
				Background(lipgloss.Color("#374151")).
				Padding(0, 1)

	CompletedItemStyle = lipgloss.NewStyle().
				Foreground(mutedColor).
				Strikethrough(true).
				Padding(0, 1)

	SelectedCompletedItemStyle = lipgloss.NewStyle().
					Foreground(secondaryColor).
					Strikethrough(true).
					Bold(true).
					Background(lipgloss.Color("#374151")).
					Padding(0, 1)

	CursorStyle = lipgloss.NewStyle().
			Foreground(accentColor).
			Bold(true)

	CheckboxStyle = lipgloss.NewStyle().
			Foreground(secondaryColor).
			Bold(true)

	InputBoxStyle = lipgloss.NewStyle().
			Padding(1, 2).
			MarginTop(1).
			MarginBottom(1)

	InstructionsStyle = lipgloss.NewStyle().
				Foreground(mutedColor).
				Italic(true).
				Padding(1, 1)

	TitleStyle = lipgloss.NewStyle().
			Foreground(primaryColor).
			Background(lipgloss.Color("#1E1B4B")).
			Bold(true).
			Padding(0, 2).
			MarginBottom(2)

	ContainerStyle = lipgloss.NewStyle().
			Padding(1, 2).
			BorderForeground(primaryColor)
)
