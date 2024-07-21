const API_URL = 'http://localhost:8080';

// DOMエレメントの取得
const todoForm = document.getElementById('todo-form');
const todoInput = document.getElementById('todo-input');
const todoList = document.getElementById('todo-list');

// Todoリストの取得と表示
async function fetchTodos() {
  try {
    const response = await fetch(`${API_URL}/todos`);
    const todos = await response.json();
    displayTodos(todos);
  } catch (error) {
    console.error('Error fetching todos:', error);
  }
}

// Todoの表示
function displayTodos(todos) {
  todoList.innerHTML = '';
  todos.forEach((todo) => {
    const li = document.createElement('li');
    li.innerHTML = `
            <span class="${todo.completed ? 'completed' : ''}">${
      todo.title
    }</span>
            <div>
                <button onclick="toggleTodo(${todo.id}, ${!todo.completed})">
                    ${todo.completed ? '未完了' : '完了'}
                </button>
                <button onclick="deleteTodo(${todo.id})">削除</button>
            </div>
        `;
    todoList.appendChild(li);
  });
}

// 新しいTodoの追加
async function addTodo(title) {
  try {
    const response = await fetch(`${API_URL}/todos`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ title, completed: false }),
    });
    if (response.ok) {
      const newTodo = await response.json();
      console.log('New todo created:', newTodo);
      fetchTodos();
      todoInput.value = '';
    } else {
      console.error('Server responded with an error:', await response.text());
    }
  } catch (error) {
    console.error('Error adding todo:', error);
  }
}

// Todoの状態を切り替え（完了/未完了）
async function toggleTodo(id, completed) {
  try {
    const response = await fetch(`${API_URL}/todos/${id}`, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ completed }),
    });
    if (response.ok) {
      fetchTodos();
    }
  } catch (error) {
    console.error('Error toggling todo:', error);
  }
}

// Todoの削除
async function deleteTodo(id) {
  try {
    const response = await fetch(`${API_URL}/todos/${id}`, {
      method: 'DELETE',
    });
    if (response.ok) {
      fetchTodos();
    }
  } catch (error) {
    console.error('Error deleting todo:', error);
  }
}

// フォームのサブミットイベントリスナー
todoForm.addEventListener('submit', (e) => {
  e.preventDefault();
  const title = todoInput.value.trim();
  if (title) {
    addTodo(title);
  }
});

// 初期ロード時にTodoリストを取得
fetchTodos();
