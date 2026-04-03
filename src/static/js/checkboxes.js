// Markdown-Checkboxen in der Detailansicht interaktiv machen (Story 36, K4)
// Ohne JS: Checkboxen bleiben disabled (nicht anklickbar, aber korrekt dargestellt).
// Mit JS: disabled wird entfernt, Checkboxen sind anklickbar (kein Speichern, nur visuell).
(function () {
  document.querySelectorAll('.markdown-content input[type="checkbox"]').forEach(function (cb) {
    cb.removeAttribute('disabled');
    cb.addEventListener('change', function (event) {
      // Kein Submit, kein Neuladen: Zustand bleibt nur visuell
      event.stopPropagation();
    });
  });
})();
