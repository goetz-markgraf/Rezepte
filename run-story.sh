echo ###############################################
echo ### implementing the next story
echo ###############################################
opencode run "@.opencode/commands/run-story.md" --model "portkey evaluation/@openrouter-eval/moonshotai/kimi-k2.5"

echo ###############################################
echo ### sending completion message to user
echo ###############################################
osascript ~/bin/send_question.applescript "Story abgeschlossen"
