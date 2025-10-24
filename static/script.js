// Mapeamento Estado -> Regional
const regionaisPorEstado = {
    'Acre': 'GR11 UO11.2',
    'Alagoas': 'GR06 UO06.1',
    'Amapá': 'GR11',
    'Amazonas': 'GR10 UO12.2',
    'Bahia': 'GR08',
    'Ceará': 'GR09',
    'Distrito Federal': 'UO001',
    'Espírito Santo': 'GR02 UO02.1',
    'Goiás': 'GR07',
    'Maranhão': 'GR10 UO10.1',
    'Mato Grosso': 'GR07 UO07.1',
    'Mato Grosso do Sul': 'GR07 UO07.2',
    'Minas Gerais': 'GR04',
    'Pará': 'GR10',
    'Paraíba': 'GR06 UO06.2',
    'Paraná': 'GR03',
    'Pernambuco': 'GR06',
    'Piauí': 'GR09 UO09.2',
    'Rio de Janeiro': 'GR02',
    'Rio Grande do Norte': 'GR09 UO9.1',
    'Rio Grande do Sul': 'GR05',
    'Rondônia': 'GR11 UO11.1',
    'Roraima': 'GR11 UO11.3',
    'Santa Catarina': 'GR03 UO03.1',
    'São Paulo': 'GR01',
    'Sergipe': 'GR08 UO08.1',
    'Tocantins': 'GR07 UO07.3'
};

// Elementos
const estadoSelect = document.getElementById('estado');
const formulario = document.getElementById('formulario');
const btnSubmit = document.getElementById('btnSubmit');
const btnLimpar = document.getElementById('btnLimpar');
const payloadTextarea = document.getElementById('payload');
const charCount = document.getElementById('charCount');
const infoRegional = document.getElementById('infoRegional');
const infoData = document.getElementById('infoData');

// Popula estados
function popularEstados() {
    const estados = Object.keys(regionaisPorEstado).sort();
    estados.forEach(estado => {
        const option = document.createElement('option');
        option.value = estado;
        option.textContent = estado;
        estadoSelect.appendChild(option);
    });
}

// Atualiza info de regional
estadoSelect.addEventListener('change', (e) => {
    const estado = e.target.value;
    if (estado) {
        const regional = regionaisPorEstado[estado];
        infoRegional.textContent = regional;
        infoRegional.style.color = 'var(--primary)';
        infoRegional.style.fontWeight = '600';
    } else {
        infoRegional.textContent = 'Será definida automaticamente';
        infoRegional.style.color = '';
        infoRegional.style.fontWeight = '';
    }
});

// Contador de caracteres
payloadTextarea.addEventListener('input', (e) => {
    const count = e.target.value.length;
    charCount.textContent = count;
    
    if (count > 500) {
        charCount.style.color = 'var(--primary)';
        charCount.style.fontWeight = '600';
    } else {
        charCount.style.color = '';
        charCount.style.fontWeight = '';
    }
});

// Atualiza data atual
function atualizarData() {
    const agora = new Date();
    const dataFormatada = agora.toLocaleDateString('pt-BR', {
        day: '2-digit',
        month: '2-digit',
        year: 'numeric'
    });
    infoData.textContent = dataFormatada;
}

// Submit do formulário
formulario.addEventListener('submit', async (e) => {
    e.preventDefault();
    
    // Loading state
    btnSubmit.classList.add('loading');
    btnSubmit.disabled = true;
    
    const dados = {
        estado: estadoSelect.value,
        responsavel: document.getElementById('responsavel').value,
        payload: payloadTextarea.value
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
            // Sucesso - mostra modal
            mostrarModal(resultado.mensagem);
            formulario.reset();
            infoRegional.textContent = 'Será definida automaticamente';
            charCount.textContent = '0';
        } else {
            // Erro - mostra toast
            mostrarToast('Erro ao Salvar', resultado.mensagem, 'error');
        }
    } catch (error) {
        mostrarToast('Erro de Conexão', 'Não foi possível conectar ao servidor', 'error');
    } finally {
        btnSubmit.classList.remove('loading');
        btnSubmit.disabled = false;
    }
});

// Botão limpar
btnLimpar.addEventListener('click', () => {
    infoRegional.textContent = 'Será definida automaticamente';
    charCount.textContent = '0';
});

// Toast
function mostrarToast(titulo, mensagem, tipo = 'success') {
    const toast = document.getElementById('toast');
    toast.className = `toast ${tipo} show`;
    toast.querySelector('.toast-title').textContent = titulo;
    toast.querySelector('.toast-message').textContent = mensagem;
    
    setTimeout(() => {
        toast.classList.remove('show');
    }, 5000);
}

function closeToast() {
    document.getElementById('toast').classList.remove('show');
}

// Modal
function mostrarModal(mensagem) {
    const modal = document.getElementById('modalConfirmacao');
    
    // Extrai ID da mensagem se houver
    const match = mensagem.match(/ID: ([a-f0-9]+)/i);
    if (match) {
        document.getElementById('modalId').textContent = match[1];
    } else {
        document.getElementById('modalId').textContent = 'Gerado com sucesso';
    }
    
    modal.classList.add('show');
}

function closeModal() {
    document.getElementById('modalConfirmacao').classList.remove('show');
}

// Fecha modal ao clicar no overlay
document.querySelector('.modal-overlay')?.addEventListener('click', closeModal);

// Inicialização
popularEstados();
atualizarData();
setInterval(atualizarData, 60000); 