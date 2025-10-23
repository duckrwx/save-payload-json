// Remove todo o mapeamento regionaisPorEstado

const estadoSelect = document.getElementById('estado');
const formulario = document.getElementById('formulario');
const mensagemDiv = document.getElementById('mensagem');

// Lista de estados
const estados = [
    'Acre', 'Alagoas', 'Amapá', 'Amazonas', 'Bahia', 'Ceará',
    'Distrito Federal', 'Espírito Santo', 'Goiás', 'Maranhão',
    'Mato Grosso', 'Mato Grosso do Sul', 'Minas Gerais', 'Pará',
    'Paraíba', 'Paraná', 'Pernambuco', 'Piauí', 'Rio de Janeiro',
    'Rio Grande do Norte', 'Rio Grande do Sul', 'Rondônia', 'Roraima',
    'Santa Catarina', 'São Paulo', 'Sergipe', 'Tocantins'
];

// Popula dropdown de estados
function popularEstados() {
    estados.forEach(estado => {
        const option = document.createElement('option');
        option.value = estado;
        option.textContent = estado;
        estadoSelect.appendChild(option);
    });
}

// Submete formulário
formulario.addEventListener('submit', async (e) => {
    e.preventDefault();
    
    const dados = {
        estado: estadoSelect.value,
        // regional: REMOVIDO - agora é automático!
        responsavel: document.getElementById('responsavel').value,
        payload: document.getElementById('payload').value
    };
    
    try {
        const response = await fetch('/api/adicionar', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(dados)
        });
        
        const resultado = await response.json();
        
        if (response.ok) {
            mostrarMensagem('✅ ' + resultado.mensagem, 'sucesso');
            formulario.reset();
        } else {
            mostrarMensagem('❌ ' + resultado.mensagem, 'erro');
        }
    } catch (error) {
        mostrarMensagem('❌ Erro ao enviar: ' + error.message, 'erro');
    }
});

function mostrarMensagem(texto, tipo) {
    mensagemDiv.textContent = texto;
    mensagemDiv.className = 'mensagem ' + tipo;
    
    setTimeout(() => {
        mensagemDiv.style.display = 'none';
    }, 5000);
}

popularEstados();