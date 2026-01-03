package cmd

import (
	"fmt"
	"os"
	"strings"

	"github.com/charmbracelet/bubbles/textarea"
	"github.com/charmbracelet/bubbles/viewport"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/lipgloss"
	"github.com/spf13/cobra"
)

// Color palette
var (
	primaryColor   = lipgloss.Color("#7C3AED") // Purple
	secondaryColor = lipgloss.Color("#10B981") // Green
	accentColor    = lipgloss.Color("#F59E0B") // Orange
	textColor      = lipgloss.Color("#E5E7EB") // Light gray
	mutedColor     = lipgloss.Color("#9CA3AF") // Muted gray
	errorColor     = lipgloss.Color("#EF4444") // Red
	bgColor        = lipgloss.Color("#1F2937") // Dark gray
)

// Styles
var (
	headerStyle = lipgloss.NewStyle().
			Foreground(primaryColor).
			Bold(true).
			Padding(0, 1).
			MarginBottom(1)

	todoItemStyle = lipgloss.NewStyle().
			Foreground(textColor).
			Padding(0, 1)

	selectedItemStyle = lipgloss.NewStyle().
				Foreground(primaryColor).
				Bold(true).
				Background(lipgloss.Color("#374151")).
				Padding(0, 1)

	completedItemStyle = lipgloss.NewStyle().
				Foreground(mutedColor).
				Strikethrough(true).
				Padding(0, 1)

	selectedCompletedItemStyle = lipgloss.NewStyle().
					Foreground(secondaryColor).
					Strikethrough(true).
					Bold(true).
					Background(lipgloss.Color("#374151")).
					Padding(0, 1)

	cursorStyle = lipgloss.NewStyle().
			Foreground(accentColor).
			Bold(true)

	checkboxStyle = lipgloss.NewStyle().
			Foreground(secondaryColor).
			Bold(true)

	inputBoxStyle = lipgloss.NewStyle().
			Padding(1, 2).
			MarginTop(1).
			MarginBottom(1)

	instructionsStyle = lipgloss.NewStyle().
				Foreground(mutedColor).
				Italic(true).
				Padding(1, 1)

	titleStyle = lipgloss.NewStyle().
			Foreground(primaryColor).
			Background(lipgloss.Color("#1E1B4B")).
			Bold(true).
			Padding(0, 2).
			MarginBottom(2)

	containerStyle = lipgloss.NewStyle().
			Padding(1, 2).
			BorderForeground(primaryColor)

	footerStyle = lipgloss.NewStyle().
			Foreground(mutedColor).
			Padding(1, 0).
			MarginTop(1)
)

type model struct {
	todo      []string
	cursor    int
	done      map[int]struct{}
	textarea  textarea.Model
	viewport  viewport.Model
	inputMode bool
	width     int
	height    int
}

func initialModel() tea.Model {
	// Create textarea
	ta := textarea.New()
	ta.Placeholder = "Enter your todo item..."
	ta.Prompt = "│ "
	ta.SetHeight(3)
	ta.SetWidth(60)
	ta.ShowLineNumbers = false
	ta.FocusedStyle.CursorLine = lipgloss.NewStyle()
	ta.BlurredStyle.Base = lipgloss.NewStyle().
		Border(lipgloss.RoundedBorder()).
		BorderForeground(mutedColor)

	// Create viewport
	vp := viewport.New(80, 20)

	return model{
		todo:      []string{},
		done:      make(map[int]struct{}),
		textarea:  ta,
		viewport:  vp,
		inputMode: false,
		width:     80,
		height:    24,
	}
}

func (m model) Init() tea.Cmd {
	return nil
}

func (m model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	var tiCmd tea.Cmd
	var vpCmd tea.Cmd

	switch msg := msg.(type) {
	case tea.WindowSizeMsg:
		m.width = msg.Width
		m.height = msg.Height
		m.textarea.SetWidth(min(60, m.width-10))
		m.viewport.Width = m.width - 4
		m.viewport.Height = m.height - 10

	case tea.KeyMsg:
		// Handle input mode
		if m.inputMode {
			switch msg.String() {
			case "esc":
				m.inputMode = false
				m.textarea.Blur()
				m.textarea.Reset()
				return m, nil
			case "ctrl+c":
				return m, tea.Quit
			case "enter":
				value := strings.TrimSpace(m.textarea.Value())
				if value != "" {
					m.todo = append(m.todo, value)
					m.textarea.Reset()
				}
				return m, nil
			default:
				m.textarea, tiCmd = m.textarea.Update(msg)
				return m, tiCmd
			}
		}

		// Handle navigation mode
		switch msg.String() {
		case "ctrl+c", "q":
			return m, tea.Quit
		case "up", "k":
			if m.cursor > 0 {
				m.cursor--
			}
		case "down", "j":
			if m.cursor < len(m.todo)-1 {
				m.cursor++
			}
		case "i", "a", "n":
			m.inputMode = true
			m.textarea.Focus()
			return m, textarea.Blink
		case " ", "x":
			if len(m.todo) > 0 && m.cursor < len(m.todo) {
				_, ok := m.done[m.cursor]
				if ok {
					delete(m.done, m.cursor)
				} else {
					m.done[m.cursor] = struct{}{}
				}
			}
		case "d":
			if len(m.todo) > 0 && m.cursor < len(m.todo) {
				// Delete current item
				delete(m.done, m.cursor)
				m.todo = append(m.todo[:m.cursor], m.todo[m.cursor+1:]...)
				// Adjust cursor position
				if m.cursor >= len(m.todo) && len(m.todo) > 0 {
					m.cursor = len(m.todo) - 1
				}
				// Adjust done map indices
				newDone := make(map[int]struct{})
				for idx := range m.done {
					if idx < m.cursor {
						newDone[idx] = struct{}{}
					} else if idx > m.cursor {
						newDone[idx-1] = struct{}{}
					}
				}
				m.done = newDone
			}
		case "r":
			// Mark all as completed
			for i := range m.todo {
				m.done[i] = struct{}{}
			}
		case "u":
			// Mark all as uncompleted
			m.done = make(map[int]struct{})
		}
	}

	if !m.inputMode {
		m.viewport, vpCmd = m.viewport.Update(msg)
	}

	return m, tea.Batch(tiCmd, vpCmd)
}

func (m model) View() string {
	// Header
	title := titleStyle.Render("Checklist")

	var content strings.Builder

	// Todo items
	if len(m.todo) == 0 {
		emptyMsg := instructionsStyle.Render("No todos yet. Press 'i' to add your first todo!")
		content.WriteString(emptyMsg)
	} else {
		for i, item := range m.todo {
			var line string
			cursor := "  "
			checkbox := "[]"

			// Determine if item is completed
			_, isCompleted := m.done[i]
			if isCompleted {
				checkbox = checkboxStyle.Render("[x]")
			}

			// Apply cursor and styling
			if m.cursor == i && !m.inputMode {
				cursor = cursorStyle.Render("▶ ")
				if isCompleted {
					line = selectedCompletedItemStyle.Render(fmt.Sprintf("%s %s", checkbox, item))
				} else {
					line = selectedItemStyle.Render(fmt.Sprintf("%s %s", checkbox, item))
				}
			} else {
				if isCompleted {
					line = completedItemStyle.Render(fmt.Sprintf("%s %s", checkbox, item))
				} else {
					line = todoItemStyle.Render(fmt.Sprintf("%s %s", checkbox, item))
				}
			}

			content.WriteString(fmt.Sprintf("%s%s\n", cursor, line))
		}
	}

	// Stats
	completed := len(m.done)
	total := len(m.todo)
	var statsMsg string
	if total > 0 {
		percentage := float64(completed) / float64(total) * 100
		statsMsg = instructionsStyle.Render(fmt.Sprintf("Progress: %d/%d completed (%.0f%%)", completed, total, percentage))
	}

	// Input area
	var inputArea string
	if m.inputMode {
		inputArea = inputBoxStyle.Render(
			fmt.Sprintf("Add new todo:\n%s", m.textarea.View()),
		)
	}

	// Instructions
	var instructions string
	if m.inputMode {
		instructions = footerStyle.Render("💡 Press Enter to add • Esc to cancel • Ctrl+C to quit")
	} else {
		keyMappings := []string{
			"i/n: add todo",
			"space/x: toggle",
			"d: delete",
			"↑↓/j/k: navigate",
			"r: complete all",
			"u: uncomplete all",
			"q: quit",
		}
		instructions = footerStyle.Render("💡 " + strings.Join(keyMappings, " • "))
	}

	// Combine everything
	var result strings.Builder
	result.WriteString(title + "\n\n")

	if statsMsg != "" {
		result.WriteString(statsMsg + "\n\n")
	}

	result.WriteString(content.String())

	if inputArea != "" {
		result.WriteString("\n" + inputArea)
	}

	result.WriteString("\n" + instructions)

	// Wrap in container
	finalContent := containerStyle.Width(min(m.width-4, 80)).Render(result.String())

	return finalContent
}

var initCmd = &cobra.Command{
	Use:   "init",
	Short: "Start pancake",
	Long:  "Start pancake to manage pull request checklist",

	Run: func(cmd *cobra.Command, args []string) {
		p := tea.NewProgram(
			initialModel(),
			tea.WithAltScreen(),
			tea.WithMouseCellMotion(),
		)
		if _, err := p.Run(); err != nil {
			fmt.Printf("Error running pancake: %v", err)
			os.Exit(1)
		}
	},
}
