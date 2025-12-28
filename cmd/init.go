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

var selectedItemStyle = lipgloss.NewStyle().Foreground(lipgloss.Color("170"))

type model struct {
	todo     []string
	cursor   int
	done     map[int]struct{}
	textarea textarea.Model
	viewport viewport.Model
}

func initialModel() tea.Model {
	vp := viewport.New(80, 20)
	ta := textarea.New()
	ta.Placeholder = "Add new check"
	ta.Prompt = "|"
	ta.SetHeight(1)
	ta.SetWidth(30)
	ta.ShowLineNumbers = false
	ta.FocusedStyle.CursorLine = lipgloss.NewStyle()
	return model{
		todo:     []string{},
		done:     make(map[int]struct{}),
		textarea: ta,
		viewport: vp,
	}
}

func (m model) Init() tea.Cmd {
	return nil
}

func (m model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	var tiCmd tea.Cmd
	var vpCmd tea.Cmd
	m.textarea, tiCmd = m.textarea.Update(msg)
	m.viewport, vpCmd = m.viewport.Update(msg)
	switch msg := msg.(type) {
	case tea.KeyMsg:

		// Cool, what was the actual key pressed?
		switch msg.String() {

		case "ctrl+c", "q":
			tea.ClearScreen()
			return m, tea.Quit
		case "up", "k":
			if m.cursor > 0 {
				m.cursor--
			}
		case "down", "j":
			if m.cursor < len(m.todo)-1 {
				m.cursor++
			}
		// add new item

		case "i":
			m.textarea.Focus()
			switch msg.Type {
			case tea.KeyCtrlC, tea.KeyEsc:
				fmt.Println(m.textarea.Value())
				return m, tea.Quit
			case tea.KeyEnter:
				m.todo = append(m.todo, m.textarea.Value())
				m.viewport.SetContent(strings.Join(m.todo, "\n"))
				m.textarea.Reset()
				// m.viewport.GotoBottom()
			}

		case " ":
			_, ok := m.done[m.cursor]
			if ok {
				delete(m.done, m.cursor)
			} else {
				m.done[m.cursor] = struct{}{}
			}
		}
	}

	return m, tea.Batch(tiCmd, vpCmd)
}

func (m model) View() string {
	s := ""

	for i, choice := range m.todo {

		cursor := " " // no cursor
		if m.cursor == i {
			cursor = selectedItemStyle.Render(">")
			choice = selectedItemStyle.Render(choice)
		}

		checked := " " // not selected
		if _, ok := m.done[i]; ok {
			checked = "√"
		}

		s += fmt.Sprintf("%s [%s] %s\n", cursor, checked, choice)
	}
	m.viewport.SetContent(strings.Join(m.todo, "\n"))

	// Send the UI for rendering
	return fmt.Sprintf("%s\n\n%s", m.viewport.View(), m.textarea.View())
}

var initCmd = &cobra.Command{
	Use:   "init",
	Short: "start checklist for the current checkout branch",
	Run: func(cmd *cobra.Command, args []string) {
		p := tea.NewProgram(initialModel(), tea.WithAltScreen())
		if _, err := p.Run(); err != nil {
			fmt.Printf("Alas, there's been an error: %v", err)
			os.Exit(1)
		}
	},
}
